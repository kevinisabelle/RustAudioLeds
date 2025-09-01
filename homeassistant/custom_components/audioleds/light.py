from __future__ import annotations

from typing import Any
import logging

from homeassistant.components.light import (
    ATTR_BRIGHTNESS,
    ATTR_RGB_COLOR,
    ColorMode,
    LightEntity,
)
from homeassistant.core import HomeAssistant
from homeassistant.helpers.entity_platform import AddEntitiesCallback
from homeassistant.helpers.update_coordinator import CoordinatorEntity
from homeassistant.config_entries import ConfigEntry

from .const import DOMAIN, DATA_COORDINATOR, DATA_API
from .coordinator import AudioLedsDataUpdateCoordinator
from .api import AudioLedsApi

_LOGGER = logging.getLogger(__name__)

PARALLEL_UPDATES = 1


async def async_setup_entry(
    hass: HomeAssistant, entry: ConfigEntry, async_add_entities: AddEntitiesCallback
) -> None:
    data = hass.data[DOMAIN][entry.entry_id]
    coordinator: AudioLedsDataUpdateCoordinator = data[DATA_COORDINATOR]
    api: AudioLedsApi = data[DATA_API]
    entities: list[LightEntity] = [
        AudioLedsPrimaryLight(coordinator, api, entry),
        AudioLedsColorSlotLight(coordinator, api, entry, slot_index=2),
        AudioLedsColorSlotLight(coordinator, api, entry, slot_index=3),
    ]
    async_add_entities(entities)


def _normalize_color(value: Any) -> tuple[int, int, int] | None:
    if isinstance(value, dict):
        try:
            r, g, b = int(value.get("r", 0)), int(value.get("g", 0)), int(value.get("b", 0))
            return max(0, min(r, 255)), max(0, min(g, 255)), max(0, min(b, 255))
        except (TypeError, ValueError):  # noqa: PERF203
            return None
    if isinstance(value, (list, tuple)) and len(value) == 3:
        try:
            r, g, b = (int(v) for v in value)
            return tuple(max(0, min(v, 255)) for v in (r, g, b))  # type: ignore[return-value]
        except (TypeError, ValueError):
            return None
    return None


def _color_to_payload(rgb: tuple[int, int, int]) -> dict[str, int]:
    r, g, b = rgb
    return {"r": r, "g": g, "b": b}


class _AudioLedsBase(CoordinatorEntity[AudioLedsDataUpdateCoordinator], LightEntity):
    _attr_supported_color_modes = {ColorMode.RGB}
    _attr_color_mode = ColorMode.RGB

    def __init__(self, coordinator: AudioLedsDataUpdateCoordinator, api: AudioLedsApi, entry: ConfigEntry) -> None:
        super().__init__(coordinator)
        self.api = api
        self.entry = entry
        self._last_rgb: tuple[int, int, int] | None = None

    @property
    def device_info(self) -> dict[str, Any]:
        info = self.coordinator.data.get("info", {}) if self.coordinator.data else {}
        identifiers = {(DOMAIN, self.entry.unique_id)} if self.entry.unique_id else {(DOMAIN, self.entry.entry_id)}
        return {
            "identifiers": identifiers,
            "name": info.get("device_name") or info.get("name") or "AudioLeds",
            "manufacturer": info.get("manufacturer", "AudioLeds"),
            "model": info.get("model", info.get("device_type", "LED Controller")),
            "sw_version": info.get("firmware") or info.get("version"),
        }

    def _color_from_info(self, key: str) -> tuple[int, int, int] | None:
        info = self.coordinator.data.get("info", {}) if self.coordinator.data else {}
        return _normalize_color(info.get(key))


class AudioLedsPrimaryLight(_AudioLedsBase):
    """Primary light: controls brightness + color1."""

    _attr_has_entity_name = True

    def __init__(self, coordinator: AudioLedsDataUpdateCoordinator, api: AudioLedsApi, entry: ConfigEntry) -> None:
        super().__init__(coordinator, api, entry)
        self._attr_unique_id = f"{entry.unique_id}_color1"
        self._last_brightness: int | None = None

    @property
    def name(self) -> str | None:
        base = self.coordinator.data.get("info", {}).get("device_name") or "AudioLeds"
        return f"{base} Primary"

    # Brightness (0-255) derived from info brightness 0..1 or 0..255
    @property
    def brightness(self) -> int | None:
        info = self.coordinator.data.get("info", {}) if self.coordinator.data else {}
        val = info.get("brightness")
        if isinstance(val, (int, float)):
            if val <= 1:
                return max(0, min(int(val * 255), 255))
            return max(0, min(int(val), 255))
        return None

    @property
    def rgb_color(self) -> tuple[int, int, int] | None:
        c = self._color_from_info("color1")
        if c:
            self._last_rgb = c
        return c or self._last_rgb

    @property
    def is_on(self) -> bool:
        b = self.brightness
        return b is not None and b > 0

    async def async_turn_on(self, **kwargs: Any) -> None:
        payload: dict[str, Any] = {}
        brightness = kwargs.get(ATTR_BRIGHTNESS)
        if brightness is None:
            brightness = self._last_brightness or self.brightness or 255
        self._last_brightness = brightness
        payload["brightness"] = round(brightness / 255.0, 3)

        rgb = kwargs.get(ATTR_RGB_COLOR)
        if rgb is not None and isinstance(rgb, (list, tuple)) and len(rgb) == 3:
            rgb_tuple = tuple(int(max(0, min(v, 255))) for v in rgb)  # type: ignore[arg-type]
            self._last_rgb = rgb_tuple
            payload["color1"] = _color_to_payload(rgb_tuple)
        elif self._last_rgb is not None:
            payload.setdefault("color1", _color_to_payload(self._last_rgb))

        _LOGGER.debug("Primary light turn_on payload: %s", payload)
        await self.api.async_post_command(payload)
        await self.coordinator.async_request_refresh()

    async def async_turn_off(self, **kwargs: Any) -> None:
        current = self.brightness
        if current:
            self._last_brightness = current
        _LOGGER.debug("Primary light turning off (brightness->0)")
        await self.api.async_post_command({"brightness": 0.0})
        await self.coordinator.async_request_refresh()


class AudioLedsColorSlotLight(_AudioLedsBase):
    """Secondary color slot (color2 or color3) as separate light entity.

    These do not control overall brightness; turning off sets slot color to black (0,0,0).
    """

    def __init__(self, coordinator: AudioLedsDataUpdateCoordinator, api: AudioLedsApi, entry: ConfigEntry, slot_index: int) -> None:
        super().__init__(coordinator, api, entry)
        assert slot_index in (2, 3)
        self.slot_index = slot_index
        self.color_key = f"color{slot_index}"
        self._attr_unique_id = f"{entry.unique_id}_{self.color_key}"

    @property
    def name(self) -> str | None:
        base = self.coordinator.data.get("info", {}).get("device_name") or "AudioLeds"
        return f"{base} Color {self.slot_index}"

    @property
    def rgb_color(self) -> tuple[int, int, int] | None:
        c = self._color_from_info(self.color_key)
        if c:
            self._last_rgb = c
        return c or self._last_rgb

    @property
    def is_on(self) -> bool:
        c = self.rgb_color
        return c is not None and any(v > 0 for v in c)

    async def async_turn_on(self, **kwargs: Any) -> None:
        rgb = kwargs.get(ATTR_RGB_COLOR)
        payload: dict[str, Any] = {}
        if rgb and isinstance(rgb, (list, tuple)) and len(rgb) == 3:
            rgb_tuple = tuple(int(max(0, min(v, 255))) for v in rgb)  # type: ignore[arg-type]
            self._last_rgb = rgb_tuple
            payload[self.color_key] = _color_to_payload(rgb_tuple)
        elif self._last_rgb is not None:
            payload[self.color_key] = _color_to_payload(self._last_rgb)
        else:
            # Default to white if nothing specified
            payload[self.color_key] = _color_to_payload((255, 255, 255))
            self._last_rgb = (255, 255, 255)
        _LOGGER.debug("Color slot %s turn_on payload: %s", self.color_key, payload)
        await self.api.async_post_command(payload)
        await self.coordinator.async_request_refresh()

    async def async_turn_off(self, **kwargs: Any) -> None:
        _LOGGER.debug("Color slot %s turning off (set black)", self.color_key)
        await self.api.async_post_command({self.color_key: {"r": 0, "g": 0, "b": 0}})
        await self.coordinator.async_request_refresh()
