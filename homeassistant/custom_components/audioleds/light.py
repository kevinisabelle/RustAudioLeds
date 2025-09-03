from __future__ import annotations

from logging import getLogger
from typing import Any

from homeassistant.components.light import (
    ATTR_BRIGHTNESS,
    ATTR_RGB_COLOR,
    ColorMode,
    LightEntity,
)
from homeassistant.config_entries import ConfigEntry
from homeassistant.core import callback, HomeAssistant
from homeassistant.helpers.aiohttp_client import async_get_clientsession
from homeassistant.helpers.device_registry import DeviceInfo
from homeassistant.helpers.entity_platform import AddEntitiesCallback
from homeassistant.helpers.update_coordinator import CoordinatorEntity

from .const import DOMAIN
from .coordinator import AudioLedsDataUpdateCoordinator

_LOGGER = getLogger(__name__)

def _get_supported_color_modes() -> set[ColorMode]:
    """Return the set of supported color modes."""
    return {ColorMode.RGB}

async def async_setup_entry(
    hass: HomeAssistant,
    entry: ConfigEntry,
    async_add_entities: AddEntitiesCallback,
) -> None:
    """Set up the AudioLEDs lights."""
    coordinator: AudioLedsDataUpdateCoordinator = hass.data[DOMAIN][entry.entry_id]
    device_info = DeviceInfo(
        identifiers={(DOMAIN, entry.unique_id)},
        name=f"AudioLEDs {entry.data['host']}",
        manufacturer="AudioLEDs",
    )

    entities = [
        AudioLedsLight(
            coordinator,
            entry,
            device_info=device_info,
            name=f"AudioLEDs {entry.data['host']}",
            unique_id=f"{entry.unique_id}_light",
        ),
        AudioLedsColorLight(
            coordinator,
            entry,
            device_info=device_info,
            name=f"AudioLEDs {entry.data['host']} Color 2",
            unique_id=f"{entry.unique_id}_color_2",
            color_key="color2",
        ),
        AudioLedsColorLight(
            coordinator,
            entry,
            device_info=device_info,
            name=f"AudioLEDs {entry.data['host']} Color 3",
            unique_id=f"{entry.unique_id}_color_3",
            color_key="color3",
        ),
    ]
    async_add_entities(entities)


class AudioLedsLight(CoordinatorEntity[AudioLedsDataUpdateCoordinator], LightEntity):
    """Representation of an AudioLEDs light."""

    _attr_color_mode = ColorMode.RGB
    _attr_supported_color_modes = _get_supported_color_modes()

    def __init__(
        self,
        coordinator: AudioLedsDataUpdateCoordinator,
        entry: ConfigEntry,
        device_info: DeviceInfo,
        name: str,
        unique_id: str,
    ) -> None:
        """Initialize the light."""
        super().__init__(coordinator)
        self._attr_name = name
        self._attr_unique_id = unique_id
        self._attr_device_info = device_info
        self._update_internal_state()

    @callback
    def _handle_coordinator_update(self) -> None:
        """Handle updated data from the coordinator."""
        self._update_internal_state()
        self.async_write_ha_state()

    def _update_internal_state(self):
        """Update our internal state from the coordinator data."""
        if self.coordinator.data and "info" in self.coordinator.data:
            info = self.coordinator.data["info"]
            brightness = info.get("brightness", 0)
            self._attr_is_on = brightness > 0
            self._attr_brightness = int(brightness * 255)
            color = info.get("color1")
            if color:
                self._attr_rgb_color = (color["r"], color["g"], color["b"])
            else:
                self._attr_rgb_color = None
        else:
            self._attr_is_on = False
            self._attr_brightness = 0
            self._attr_rgb_color = None

    async def async_turn_on(self, **kwargs: Any) -> None:
        """Turn the light on."""
        data = {}
        if ATTR_BRIGHTNESS in kwargs:
            data["brightness"] = kwargs[ATTR_BRIGHTNESS] / 255.0
        else:
            # If brightness is not specified, HA sends turn_on without it.
            # We'll ensure brightness is set to full if it's currently off.
            if not self.is_on:
                data["brightness"] = 1.0

        if ATTR_RGB_COLOR in kwargs:
            rgb = kwargs[ATTR_RGB_COLOR]
            data["color1"] = {"r": rgb[0], "g": rgb[1], "b": rgb[2]}

        if not data:
            # If no specific attributes are sent, just turn on
            data["brightness"] = 1.0

        session = async_get_clientsession(self.coordinator.hass)
        url = f"{self.coordinator.api_url}/command"
        try:
            async with session.post(
                url, json=data, ssl=self.coordinator.verify_ssl
            ) as response:
                if response.status == 200:
                    await self.coordinator.async_request_refresh()
                else:
                    _LOGGER.error("Failed to turn on AudioLEDs: %s", response.status)
        except Exception as e:
            _LOGGER.error("Failed to turn on AudioLEDs: %s", e)

    async def async_turn_off(self, **kwargs: Any) -> None:
        """Turn the light off."""
        data = {"brightness": 0}
        session = async_get_clientsession(self.coordinator.hass)
        url = f"{self.coordinator.api_url}/command"
        try:
            async with session.post(
                url, json=data, ssl=self.coordinator.verify_ssl
            ) as response:
                if response.status == 200:
                    await self.coordinator.async_request_refresh()
                else:
                    _LOGGER.error("Failed to turn off AudioLEDs: %s", response.status)
        except Exception as e:
            _LOGGER.error("Failed to turn off AudioLEDs: %s", e)


class AudioLedsColorLight(
    CoordinatorEntity[AudioLedsDataUpdateCoordinator], LightEntity
):
    """Representation of an AudioLEDs color-only light."""

    _attr_color_mode = ColorMode.RGB
    _attr_supported_color_modes = {ColorMode.RGB}
    _attr_is_on = True  # This light is always on, only color can be changed

    def __init__(
        self,
        coordinator: AudioLedsDataUpdateCoordinator,
        entry: ConfigEntry,
        device_info: DeviceInfo,
        name: str,
        unique_id: str,
        color_key: str,
    ) -> None:
        """Initialize the color light."""
        super().__init__(coordinator)
        self._attr_name = name
        self._attr_unique_id = unique_id
        self._attr_device_info = device_info
        self._color_key = color_key
        self._update_internal_state()

    @callback
    def _handle_coordinator_update(self) -> None:
        """Handle updated data from the coordinator."""
        self._update_internal_state()
        self.async_write_ha_state()

    def _update_internal_state(self):
        """Update our internal state from the coordinator data."""
        if self.coordinator.data and "info" in self.coordinator.data:
            color = self.coordinator.data["info"].get(self._color_key)
            if color:
                self._attr_rgb_color = (color["r"], color["g"], color["b"])
            else:
                self._attr_rgb_color = None
        else:
            self._attr_rgb_color = None

    async def async_turn_on(self, **kwargs: Any) -> None:
        """Change the color."""
        if ATTR_RGB_COLOR not in kwargs:
            return

        rgb = kwargs[ATTR_RGB_COLOR]
        data = {self._color_key: {"r": rgb[0], "g": rgb[1], "b": rgb[2]}}

        session = async_get_clientsession(self.coordinator.hass)
        url = f"{self.coordinator.api_url}/command"
        try:
            async with session.post(
                url, json=data, ssl=self.coordinator.verify_ssl
            ) as response:
                if response.status == 200:
                    await self.coordinator.async_request_refresh()
                else:
                    _LOGGER.error(
                        "Failed to set %s: %s", self._color_key, response.status
                    )
        except Exception as e:
            _LOGGER.error("Failed to set %s: %s", self._color_key, e)

    async def async_turn_off(self, **kwargs: Any) -> None:
        """This entity cannot be turned off."""
        pass
