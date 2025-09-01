from __future__ import annotations

from typing import Any
import logging

from homeassistant.components.light import (
    ATTR_BRIGHTNESS,
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
    async_add_entities([AudioLedsLight(coordinator, api, entry)])


class AudioLedsLight(CoordinatorEntity[AudioLedsDataUpdateCoordinator], LightEntity):
    _attr_has_entity_name = True
    _attr_name = None  # Use device name
    _attr_supported_color_modes = {ColorMode.BRIGHTNESS}
    _attr_color_mode = ColorMode.BRIGHTNESS
    _attr_unique_id: str

    def __init__(self, coordinator: AudioLedsDataUpdateCoordinator, api: AudioLedsApi, entry: ConfigEntry) -> None:
        super().__init__(coordinator)
        self.api = api
        self.entry = entry
        self._attr_unique_id = f"{entry.unique_id}_light"
        self._last_brightness: int | None = None

    @property
    def name(self) -> str | None:
        info = self.coordinator.data.get("info", {}) if self.coordinator.data else {}
        device_name = info.get("device_name") or info.get("name") or "AudioLeds"
        return device_name

    @property
    def is_on(self) -> bool:
        b = self.brightness
        return b is not None and b > 0

    @property
    def brightness(self) -> int | None:
        info = self.coordinator.data.get("info", {}) if self.coordinator.data else {}
        val = info.get("brightness")
        if isinstance(val, (int, float)):
            if val <= 1:
                return int(val * 255)
            return int(val)
        return None

    async def async_turn_on(self, **kwargs: Any) -> None:
        brightness = kwargs.get(ATTR_BRIGHTNESS)
        if brightness is None:
            # Restore previous or full brightness
            brightness = self._last_brightness or 255
        self._last_brightness = brightness
        # Convert to 0-1 float
        fractional = round(brightness / 255.0, 3)
        await self.api.async_post_command({"brightness": fractional})
        await self.coordinator.async_request_refresh()

    async def async_turn_off(self, **kwargs: Any) -> None:  # noqa: D401
        current = self.brightness
        if current:
            self._last_brightness = current
        await self.api.async_post_command({"brightness": 0.0})
        await self.coordinator.async_request_refresh()

    @property
    def device_info(self) -> dict[str, Any]:
        info = self.coordinator.data.get("info", {}) if self.coordinator.data else {}
        identifiers = {(DOMAIN, self.entry.unique_id)} if self.entry.unique_id else {(DOMAIN, self.entry.entry_id)}
        return {
            "identifiers": identifiers,
            "name": self.name,
            "manufacturer": info.get("manufacturer", "AudioLeds"),
            "model": info.get("model", info.get("device_type", "LED Controller")),
            "sw_version": info.get("firmware") or info.get("version"),
        }

