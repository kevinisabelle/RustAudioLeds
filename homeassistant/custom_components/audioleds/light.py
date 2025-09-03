from __future__ import annotations

from typing import Any

from homeassistant.components.light import (
    ATTR_BRIGHTNESS,
    ATTR_RGB_COLOR,
    ColorMode,
    LightEntity,
)
from homeassistant.helpers.update_coordinator import CoordinatorEntity

from .coordinator import AudioLedsDataUpdateCoordinator

PARALLEL_UPDATES = 0

_LOGGER = getLogger(__name__)

def _get_supported_color_modes() -> set[ColorMode]:
    """Return the set of supported color modes."""
    return {ColorMode.RGB}

class AudioLedsLight(CoordinatorEntity[AudioLedsDataUpdateCoordinator], LightEntity):
    """Representation of an AudioLEDs light."""

    _attr_color_mode = ColorMode.RGB
    _attr_supported_color_modes = _get_supported_color_modes()

    def __init__(
        self,
        coordinator: AudioLedsDataUpdateCoordinator,
        **kwargs: Any,
    ) -> None:
        """Initialize the light."""
        super().__init__(coordinator)
        self._attr_name = kwargs.get("name")
        self._attr_unique_id = kwargs.get("unique_id")
        self._attr_device_info = kwargs.get("device_info")
        self._update_internal_state()

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

        session = self.coordinator.hass.helpers.aiohttp_client.async_get_clientsession()
        url = f"{self.coordinator.api_url}/command"
        try:
            async with session.post(url, json=data) as response:
                if response.status == 200:
                    await self.coordinator.async_request_refresh()
                else:
                    _LOGGER.error("Failed to turn on AudioLEDs: %s", response.status)
        except Exception as e:
            _LOGGER.error("Failed to turn on AudioLEDs: %s", e)

    async def async_turn_off(self, **kwargs: Any) -> None:
        """Turn the light off."""
        data = {"brightness": 0}
        session = self.coordinator.hass.helpers.aiohttp_client.async_get_clientsession()
        url = f"{self.coordinator.api_url}/command"
        try:
            async with session.post(url, json=data) as response:
                if response.status == 200:
                    await self.coordinator.async_request_refresh()
                else:
                    _LOGGER.error("Failed to turn off AudioLEDs: %s", response.status)
        except Exception as e:
            _LOGGER.error("Failed to turn off AudioLEDs: %s", e)
