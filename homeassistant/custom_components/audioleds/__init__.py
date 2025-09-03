"""The AudioLEDs integration."""
from __future__ import annotations

import logging

import voluptuous as vol
from homeassistant.config_entries import ConfigEntry
from homeassistant.const import Platform, CONF_HOST, CONF_PORT
from homeassistant.core import HomeAssistant, ServiceCall
from homeassistant.helpers import config_validation as cv, device_registry as dr, entity_registry as er
from homeassistant.helpers.aiohttp_client import async_get_clientsession

from .const import DOMAIN
from .coordinator import AudioLedsDataUpdateCoordinator

_LOGGER = logging.getLogger(__name__)

PLATFORMS: list[Platform] = [
    Platform.LIGHT,
    Platform.SELECT,
    Platform.NUMBER,
    Platform.SENSOR,
]

SERVICE_SAVE_PRESET = "save_preset"
SERVICE_ACTIVATE_PRESET = "activate_preset"
SERVICE_DELETE_PRESET = "delete_preset"

SERVICE_SAVE_PRESET_SCHEMA = vol.Schema(
    {
        vol.Required("entity_id"): cv.entity_id,
        vol.Required("preset_index"): vol.All(vol.Coerce(int), vol.Range(min=0, max=19)),
        vol.Required("preset_name"): cv.string,
    }
)
SERVICE_ACTIVATE_PRESET_SCHEMA = vol.Schema(
    {
        vol.Required("entity_id"): cv.entity_id,
        vol.Required("preset_index"): vol.All(vol.Coerce(int), vol.Range(min=0, max=19)),
    }
)
SERVICE_DELETE_PRESET_SCHEMA = vol.Schema(
    {
        vol.Required("entity_id"): cv.entity_id,
        vol.Required("preset_index"): vol.All(vol.Coerce(int), vol.Range(min=0, max=19)),
    }
)


async def async_setup_entry(hass: HomeAssistant, entry: ConfigEntry) -> bool:
    """Set up AudioLEDs from a config entry."""
    hass.data.setdefault(DOMAIN, {})

    coordinator = AudioLedsDataUpdateCoordinator(
        hass,
        host=entry.data[CONF_HOST],
        port=entry.data[CONF_PORT],
    )

    await coordinator.async_config_entry_first_refresh()

    hass.data[DOMAIN][entry.entry_id] = coordinator

    if not hasattr(hass.data[DOMAIN], "services_registered"):
        hass.data[DOMAIN].services_registered = True
        await async_setup_services(hass)

    await hass.config_entries.async_forward_entry_setups(entry, PLATFORMS)

    return True


async def async_setup_services(hass: HomeAssistant) -> None:
    """Set up the services for the AudioLEDs integration."""

    async def get_coordinator_from_entity_id(
        entity_id: str,
    ) -> AudioLedsDataUpdateCoordinator | None:
        entity_registry = er.async_get(hass)
        entity = entity_registry.async_get(entity_id)
        if not entity:
            _LOGGER.error("Entity not found: %s", entity_id)
            return None

        if entity.config_entry_id in hass.data[DOMAIN]:
            return hass.data[DOMAIN][entity.config_entry_id]

        _LOGGER.error("No coordinator found for entity: %s", entity_id)
        return None

    async def save_preset(call: ServiceCall) -> None:
        """Service to save the current preset."""
        entity_id = call.data["entity_id"]
        preset_index = call.data["preset_index"]
        preset_name = call.data["preset_name"]

        coordinator = await get_coordinator_from_entity_id(entity_id)
        if not coordinator:
            return

        session = async_get_clientsession(hass)
        url = f"{coordinator.api_url}/presets/save_current"
        try:
            async with session.post(
                url, json={"index": preset_index, "name": preset_name}
            ) as response:
                if response.status == 201:
                    await coordinator.async_request_refresh()
                else:
                    _LOGGER.error(
                        "Failed to save preset: %s - %s",
                        response.status,
                        await response.text(),
                    )
        except Exception as e:
            _LOGGER.error("Failed to save preset: %s", e)

    async def activate_preset(call: ServiceCall) -> None:
        """Service to activate a preset."""
        entity_id = call.data["entity_id"]
        preset_index = call.data["preset_index"]

        coordinator = await get_coordinator_from_entity_id(entity_id)
        if not coordinator:
            return

        session = async_get_clientsession(hass)
        url = f"{coordinator.api_url}/presets/{preset_index}/activate"
        try:
            async with session.post(url) as response:
                if response.status == 200:
                    await coordinator.async_request_refresh()
                else:
                    _LOGGER.error(
                        "Failed to activate preset: %s - %s",
                        response.status,
                        await response.text(),
                    )
        except Exception as e:
            _LOGGER.error("Failed to activate preset: %s", e)

    async def delete_preset(call: ServiceCall) -> None:
        """Service to delete a preset."""
        entity_id = call.data["entity_id"]
        preset_index = call.data["preset_index"]

        coordinator = await get_coordinator_from_entity_id(entity_id)
        if not coordinator:
            return

        session = async_get_clientsession(hass)
        url = f"{coordinator.api_url}/presets/{preset_index}"
        try:
            async with session.delete(url) as response:
                if response.status == 204:
                    await coordinator.async_request_refresh()
                else:
                    _LOGGER.error(
                        "Failed to delete preset: %s - %s",
                        response.status,
                        await response.text(),
                    )
        except Exception as e:
            _LOGGER.error("Failed to delete preset: %s", e)

    hass.services.async_register(
        DOMAIN,
        SERVICE_SAVE_PRESET,
        save_preset,
        schema=SERVICE_SAVE_PRESET_SCHEMA,
    )
    hass.services.async_register(
        DOMAIN,
        SERVICE_ACTIVATE_PRESET,
        activate_preset,
        schema=SERVICE_ACTIVATE_PRESET_SCHEMA,
    )
    hass.services.async_register(
        DOMAIN,
        SERVICE_DELETE_PRESET,
        delete_preset,
        schema=SERVICE_DELETE_PRESET_SCHEMA,
    )


async def async_unload_entry(hass: HomeAssistant, entry: ConfigEntry) -> bool:
    """Unload a config entry."""
    if unload_ok := await hass.config_entries.async_unload_platforms(entry, PLATFORMS):
        hass.data[DOMAIN].pop(entry.entry_id)
        if not hass.data[DOMAIN]:
            # No more entries, unregister services
            hass.services.async_remove(DOMAIN, SERVICE_SAVE_PRESET)
            hass.services.async_remove(DOMAIN, SERVICE_ACTIVATE_PRESET)
            hass.services.async_remove(DOMAIN, SERVICE_DELETE_PRESET)
            if hasattr(hass.data[DOMAIN], "services_registered"):
                delattr(hass.data[DOMAIN], "services_registered")

    return unload_ok
