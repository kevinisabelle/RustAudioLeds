"""Number platform for AudioLEDs."""
from __future__ import annotations

import logging
from typing import Any

from homeassistant.components.number import NumberEntity
from homeassistant.config_entries import ConfigEntry
from homeassistant.core import HomeAssistant, callback
from homeassistant.helpers.entity import DeviceInfo
from homeassistant.helpers.entity_platform import AddEntitiesCallback
from homeassistant.helpers.update_coordinator import CoordinatorEntity

from .const import DOMAIN
from .coordinator import AudioLedsDataUpdateCoordinator

_LOGGER = logging.getLogger(__name__)


async def async_setup_entry(
    hass: HomeAssistant,
    entry: ConfigEntry,
    async_add_entities: AddEntitiesCallback,
) -> None:
    """Set up the AudioLEDs numbers."""
    coordinator: AudioLedsDataUpdateCoordinator = hass.data[DOMAIN][entry.entry_id]
    entities = [
        AudioLedsGainNumber(coordinator, entry),
        AudioLedsSmoothSizeNumber(coordinator, entry),
        AudioLedsSkewNumber(coordinator, entry),
    ]
    async_add_entities(entities)


class AudioLedsGainNumber(CoordinatorEntity[AudioLedsDataUpdateCoordinator], NumberEntity):
    """Representation of a gain number entity."""

    def __init__(
        self, coordinator: AudioLedsDataUpdateCoordinator, entry: ConfigEntry
    ) -> None:
        """Initialize the number entity."""
        super().__init__(coordinator)
        self._attr_name = f"AudioLEDs {entry.data['host']} Gain"
        self._attr_unique_id = f"{entry.unique_id}_gain"
        self._attr_native_min_value = 0
        self._attr_native_max_value = 10
        self._attr_native_step = 0.1
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
            self._attr_native_value = self.coordinator.data["info"].get("gain")
        else:
            self._attr_native_value = None

    async def async_set_native_value(self, value: float) -> None:
        """Update the current value."""
        session = self.coordinator.hass.helpers.aiohttp_client.async_get_clientsession()
        url = f"{self.coordinator.api_url}/command"
        try:
            async with session.post(url, json={"gain": value}) as response:
                if response.status == 200:
                    await self.coordinator.async_request_refresh()
                else:
                    _LOGGER.error("Failed to set gain: %s", response.status)
        except Exception as e:
            _LOGGER.error("Failed to set gain: %s", e)


class AudioLedsSmoothSizeNumber(
    CoordinatorEntity[AudioLedsDataUpdateCoordinator], NumberEntity
):
    """Representation of a smooth size number entity."""

    def __init__(
        self, coordinator: AudioLedsDataUpdateCoordinator, entry: ConfigEntry
    ) -> None:
        """Initialize the number entity."""
        super().__init__(coordinator)
        self._attr_name = f"AudioLEDs {entry.data['host']} Smooth Size"
        self._attr_unique_id = f"{entry.unique_id}_smooth_size"
        self._attr_native_min_value = 1
        self._attr_native_max_value = 128
        self._attr_native_step = 1
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
            self._attr_native_value = self.coordinator.data["info"].get("smooth_size")
        else:
            self._attr_native_value = None

    async def async_set_native_value(self, value: float) -> None:
        """Update the current value."""
        session = self.coordinator.hass.helpers.aiohttp_client.async_get_clientsession()
        url = f"{self.coordinator.api_url}/command"
        try:
            async with session.post(url, json={"smooth_size": int(value)}) as response:
                if response.status == 200:
                    await self.coordinator.async_request_refresh()
                else:
                    _LOGGER.error("Failed to set smooth size: %s", response.status)
        except Exception as e:
            _LOGGER.error("Failed to set smooth size: %s", e)


class AudioLedsSkewNumber(
    CoordinatorEntity[AudioLedsDataUpdateCoordinator], NumberEntity
):
    """Representation of a skew number entity."""

    def __init__(
        self, coordinator: AudioLedsDataUpdateCoordinator, entry: ConfigEntry
    ) -> None:
        """Initialize the number entity."""
        super().__init__(coordinator)
        self._attr_name = f"AudioLEDs {entry.data['host']} Skew"
        self._attr_unique_id = f"{entry.unique_id}_skew"
        self._attr_native_min_value = 0
        self._attr_native_max_value = 10
        self._attr_native_step = 0.1
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
            self._attr_native_value = self.coordinator.data["info"].get("skew")
        else:
            self._attr_native_value = None

    async def async_set_native_value(self, value: float) -> None:
        """Update the current value."""
        session = self.coordinator.hass.helpers.aiohttp_client.async_get_clientsession()
        url = f"{self.coordinator.api_url}/command"
        try:
            async with session.post(url, json={"skew": value}) as response:
                if response.status == 200:
                    await self.coordinator.async_request_refresh()
                else:
                    _LOGGER.error("Failed to set skew: %s", response.status)
        except Exception as e:
            _LOGGER.error("Failed to set skew: %s", e)

