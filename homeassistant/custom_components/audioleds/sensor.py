"""Sensor platform for AudioLEDs."""
from __future__ import annotations

import logging

from homeassistant.components.sensor import SensorEntity
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
    """Set up the AudioLEDs sensors."""
    coordinator: AudioLedsDataUpdateCoordinator = hass.data[DOMAIN][entry.entry_id]
    async_add_entities([AudioLedsPresetSensor(coordinator, entry)])


class AudioLedsPresetSensor(
    CoordinatorEntity[AudioLedsDataUpdateCoordinator], SensorEntity
):
    """Representation of a preset sensor."""

    def __init__(
        self, coordinator: AudioLedsDataUpdateCoordinator, entry: ConfigEntry
    ) -> None:
        """Initialize the sensor."""
        super().__init__(coordinator)
        self._attr_name = f"AudioLEDs {entry.data['host']} Presets"
        self._attr_unique_id = f"{entry.unique_id}_presets"
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
        """Update our internal state from the coordinator data."""
        if self.coordinator.data and "presets" in self.coordinator.data:
            presets = self.coordinator.data["presets"]
            self._attr_native_value = len(presets)
            self._attr_extra_state_attributes = {"presets": presets}
        else:
            self._attr_native_value = 0
            self._attr_extra_state_attributes = {"presets": []}

