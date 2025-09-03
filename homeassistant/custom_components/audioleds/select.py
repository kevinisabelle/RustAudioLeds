"""Select platform for AudioLEDs."""
from __future__ import annotations

import logging
from typing import Any

from homeassistant.components.select import SelectEntity
from homeassistant.config_entries import ConfigEntry
from homeassistant.core import HomeAssistant, callback
from homeassistant.helpers.entity import DeviceInfo
from homeassistant.helpers.entity_platform import AddEntitiesCallback
from homeassistant.helpers.update_coordinator import CoordinatorEntity

from .const import DOMAIN
from .coordinator import AudioLedsDataUpdateCoordinator

_LOGGER = logging.getLogger(__name__)

DISPLAY_MODES = ["Spectrum", "Oscilloscope", "ColorGradient"]
ANIMATION_MODES = [
    "Full",
    "FullWithMax",
    "Points",
    "FullMiddle",
    "FullMiddleWithMax",
    "PointsMiddle",
]
FFT_SIZES = ["256", "512", "1024", "2048", "4096", "8192"]


async def async_setup_entry(
    hass: HomeAssistant,
    entry: ConfigEntry,
    async_add_entities: AddEntitiesCallback,
) -> None:
    """Set up the AudioLEDs selects."""
    coordinator: AudioLedsDataUpdateCoordinator = hass.data[DOMAIN][entry.entry_id]
    entities = [
        AudioLedsDisplayModeSelect(coordinator, entry),
        AudioLedsAnimationModeSelect(coordinator, entry),
        AudioLedsPresetSelect(coordinator, entry),
        AudioLedsFftSizeSelect(coordinator, entry),
    ]
    async_add_entities(entities)


class AudioLedsDisplayModeSelect(
    CoordinatorEntity[AudioLedsDataUpdateCoordinator], SelectEntity
):
    """Representation of a display mode select entity."""

    def __init__(
        self, coordinator: AudioLedsDataUpdateCoordinator, entry: ConfigEntry
    ) -> None:
        """Initialize the select entity."""
        super().__init__(coordinator)
        self._attr_name = f"AudioLEDs {entry.data['host']} Display Mode"
        self._attr_unique_id = f"{entry.unique_id}_display_mode"
        self._attr_options = DISPLAY_MODES
        self._attr_device_info = DeviceInfo(
            identifiers={(DOMAIN, entry.unique_id)},
            name=f"AudioLEDs {entry.data['host']}",
            manufacturer="AudioLEDs",
        )
        self._update_internal_state()

    @callback
    def _handle_coordinator_update(self) -> None:
        """Handle updated data from the coordinator."""
        self._update_internal_state()
        self.async_write_ha_state()

    def _update_internal_state(self):
        if self.coordinator.data and "info" in self.coordinator.data:
            self._attr_current_option = self.coordinator.data["info"].get("display_mode")
        else:
            self._attr_current_option = None

    async def async_select_option(self, option: str) -> None:
        """Change the selected option."""
        session = self.coordinator.hass.helpers.aiohttp_client.async_get_clientsession()
        url = f"{self.coordinator.api_url}/command"
        try:
            async with session.post(
                url, json={"display_mode": option}, ssl=self.coordinator.verify_ssl
            ) as response:
                if response.status == 200:
                    await self.coordinator.async_request_refresh()
                else:
                    _LOGGER.error(
                        "Failed to set display mode: %s", response.status
                    )
        except Exception as e:
            _LOGGER.error("Failed to set display mode: %s", e)


class AudioLedsAnimationModeSelect(
    CoordinatorEntity[AudioLedsDataUpdateCoordinator], SelectEntity
):
    """Representation of an animation mode select entity."""

    def __init__(
        self, coordinator: AudioLedsDataUpdateCoordinator, entry: ConfigEntry
    ) -> None:
        """Initialize the select entity."""
        super().__init__(coordinator)
        self._attr_name = f"AudioLEDs {entry.data['host']} Animation Mode"
        self._attr_unique_id = f"{entry.unique_id}_animation_mode"
        self._attr_options = ANIMATION_MODES
        self._attr_device_info = DeviceInfo(
            identifiers={(DOMAIN, entry.unique_id)},
            name=f"AudioLEDs {entry.data['host']}",
            manufacturer="AudioLEDs",
        )
        self._update_internal_state()

    @callback
    def _handle_coordinator_update(self) -> None:
        """Handle updated data from the coordinator."""
        self._update_internal_state()
        self.async_write_ha_state()

    def _update_internal_state(self):
        if self.coordinator.data and "info" in self.coordinator.data:
            self._attr_current_option = self.coordinator.data["info"].get(
                "animation_mode"
            )
        else:
            self._attr_current_option = None

    async def async_select_option(self, option: str) -> None:
        """Change the selected option."""
        session = self.coordinator.hass.helpers.aiohttp_client.async_get_clientsession()
        url = f"{self.coordinator.api_url}/command"
        try:
            async with session.post(
                url, json={"animation_mode": option}, ssl=self.coordinator.verify_ssl
            ) as response:
                if response.status == 200:
                    await self.coordinator.async_request_refresh()
                else:
                    _LOGGER.error(
                        "Failed to set animation mode: %s", response.status
                    )
        except Exception as e:
            _LOGGER.error("Failed to set animation mode: %s", e)


class AudioLedsPresetSelect(
    CoordinatorEntity[AudioLedsDataUpdateCoordinator], SelectEntity
):
    """Representation of a preset select entity."""

    def __init__(
        self, coordinator: AudioLedsDataUpdateCoordinator, entry: ConfigEntry
    ) -> None:
        """Initialize the select entity."""
        super().__init__(coordinator)
        self._attr_name = f"AudioLEDs {entry.data['host']} Preset"
        self._attr_unique_id = f"{entry.unique_id}_preset"
        self._attr_device_info = DeviceInfo(
            identifiers={(DOMAIN, entry.unique_id)},
            name=f"AudioLEDs {entry.data['host']}",
            manufacturer="AudioLEDs",
        )
        self._update_internal_state()

    @callback
    def _handle_coordinator_update(self) -> None:
        """Handle updated data from the coordinator."""
        self._update_internal_state()
        self.async_write_ha_state()

    def _update_internal_state(self):
        if self.coordinator.data and "presets" in self.coordinator.data:
            presets = self.coordinator.data["presets"]
            self._attr_options = [preset["name"] for preset in presets]
            active_preset_index = self.coordinator.data["info"].get("active_preset")
            if active_preset_index is not None:
                for preset in presets:
                    if preset["index"] == active_preset_index:
                        self._attr_current_option = preset["name"]
                        break
                else:
                    self._attr_current_option = None
            else:
                self._attr_current_option = None
        else:
            self._attr_options = []
            self._attr_current_option = None

    async def async_select_option(self, option: str) -> None:
        """Change the selected option."""
        if self.coordinator.data and "presets" in self.coordinator.data:
            presets = self.coordinator.data["presets"]
            preset_id = None
            for preset in presets:
                if preset["name"] == option:
                    preset_id = preset["index"]
                    break

            if preset_id is not None:
                session = self.coordinator.hass.helpers.aiohttp_client.async_get_clientsession()
                url = f"{self.coordinator.api_url}/presets/{preset_id}/activate"
                try:
                    async with session.post(url, ssl=self.coordinator.verify_ssl) as response:
                        if response.status == 200:
                            await self.coordinator.async_request_refresh()
                        else:
                            _LOGGER.error(
                                "Failed to activate preset: %s", response.status
                            )
                except Exception as e:
                    _LOGGER.error("Failed to activate preset: %s", e)


class AudioLedsFftSizeSelect(
    CoordinatorEntity[AudioLedsDataUpdateCoordinator], SelectEntity
):
    """Representation of an FFT size select entity."""

    def __init__(
        self, coordinator: AudioLedsDataUpdateCoordinator, entry: ConfigEntry
    ) -> None:
        """Initialize the select entity."""
        super().__init__(coordinator)
        self._attr_name = f"AudioLEDs {entry.data['host']} FFT Size"
        self._attr_unique_id = f"{entry.unique_id}_fft_size"
        self._attr_options = FFT_SIZES
        self._attr_device_info = DeviceInfo(
            identifiers={(DOMAIN, entry.unique_id)},
            name=f"AudioLEDs {entry.data['host']}",
            manufacturer="AudioLEDs",
        )
        self._update_internal_state()

    @callback
    def _handle_coordinator_update(self) -> None:
        """Handle updated data from the coordinator."""
        self._update_internal_state()
        self.async_write_ha_state()

    def _update_internal_state(self):
        if self.coordinator.data and "info" in self.coordinator.data:
            fft_size = self.coordinator.data["info"].get("fft_size")
            self._attr_current_option = str(fft_size) if fft_size else None
        else:
            self._attr_current_option = None

    async def async_select_option(self, option: str) -> None:
        """Change the selected option."""
        session = self.coordinator.hass.helpers.aiohttp_client.async_get_clientsession()
        url = f"{self.coordinator.api_url}/command"
        try:
            async with session.post(
                url, json={"fft_size": int(option)}, ssl=self.coordinator.verify_ssl
            ) as response:
                if response.status == 200:
                    await self.coordinator.async_request_refresh()
                else:
                    _LOGGER.error("Failed to set FFT size: %s", response.status)
        except Exception as e:
            _LOGGER.error("Failed to set FFT size: %s", e)
