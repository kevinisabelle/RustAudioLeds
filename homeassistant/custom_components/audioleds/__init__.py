from __future__ import annotations

import logging
from typing import Any

from homeassistant.config_entries import ConfigEntry
from homeassistant.core import HomeAssistant, ServiceCall
from homeassistant.helpers.typing import ConfigType
import voluptuous as vol

from .api import AudioLedsApi, AudioLedsConfig
from .const import (
    CONF_HOST,
    CONF_PORT,
    CONF_TOKEN,
    CONF_VERIFY_SSL,
    DATA_API,
    DATA_COORDINATOR,
    DOMAIN,
    PLATFORMS,
    SERVICE_SEND_COMMAND,
    SERVICE_CREATE_PRESET,
    SERVICE_ACTIVATE_PRESET,
    SERVICE_DELETE_PRESET,
    ATTR_PRESET_ID,
)
from .coordinator import AudioLedsDataUpdateCoordinator

_LOGGER = logging.getLogger(__name__)

SEND_COMMAND_SCHEMA = vol.Schema({vol.Required("data"): dict})
CREATE_PRESET_SCHEMA = vol.Schema({vol.Required("data"): dict})
ACTIVATE_PRESET_SCHEMA = vol.Schema({vol.Required(ATTR_PRESET_ID): vol.Coerce(int)})
DELETE_PRESET_SCHEMA = vol.Schema({vol.Required(ATTR_PRESET_ID): vol.Coerce(int)})


async def async_setup(hass: HomeAssistant, config: ConfigType) -> bool:  # noqa: D401
    """Set up via config entries only."""
    return True


async def async_setup_entry(hass: HomeAssistant, entry: ConfigEntry) -> bool:
    hass.data.setdefault(DOMAIN, {})

    conf = entry.data
    api = AudioLedsApi(
        hass.helpers.aiohttp_client.async_get_clientsession(hass),
        AudioLedsConfig(
            host=conf[CONF_HOST],
            port=conf[CONF_PORT],
            token=conf[CONF_TOKEN],
            verify_ssl=conf.get(CONF_VERIFY_SSL, False),
        ),
    )

    coordinator = AudioLedsDataUpdateCoordinator(hass, api)
    await coordinator.async_config_entry_first_refresh()

    hass.data[DOMAIN][entry.entry_id] = {DATA_API: api, DATA_COORDINATOR: coordinator}

    # Register services (once). Attach to hass.data marker to avoid duplicates.
    if not hass.services.has_service(DOMAIN, SERVICE_SEND_COMMAND):
        _LOGGER.debug("Registering AudioLeds services")

        async def _handle_send_command(call: ServiceCall) -> None:
            data = call.data.get("data", {})
            _LOGGER.debug("Service send_command: %s", data)
            await api.async_post_command(data)
            await coordinator.async_request_refresh()

        async def _handle_create_preset(call: ServiceCall) -> None:
            data = call.data.get("data", {})
            _LOGGER.debug("Service create_preset: %s", data)
            await api.async_create_preset(data)
            await coordinator.async_request_refresh()

        async def _handle_activate_preset(call: ServiceCall) -> None:
            preset_id = call.data[ATTR_PRESET_ID]
            _LOGGER.debug("Service activate_preset: %s", preset_id)
            await api.async_activate_preset(preset_id)
            await coordinator.async_request_refresh()

        async def _handle_delete_preset(call: ServiceCall) -> None:
            preset_id = call.data[ATTR_PRESET_ID]
            _LOGGER.debug("Service delete_preset: %s", preset_id)
            await api.async_delete_preset(preset_id)
            await coordinator.async_request_refresh()

        hass.services.async_register(
            DOMAIN, SERVICE_SEND_COMMAND, _handle_send_command, schema=SEND_COMMAND_SCHEMA
        )
        hass.services.async_register(
            DOMAIN, SERVICE_CREATE_PRESET, _handle_create_preset, schema=CREATE_PRESET_SCHEMA
        )
        hass.services.async_register(
            DOMAIN, SERVICE_ACTIVATE_PRESET, _handle_activate_preset, schema=ACTIVATE_PRESET_SCHEMA
        )
        hass.services.async_register(
            DOMAIN, SERVICE_DELETE_PRESET, _handle_delete_preset, schema=DELETE_PRESET_SCHEMA
        )

    await hass.config_entries.async_forward_entry_setups(entry, PLATFORMS)
    return True


async def async_unload_entry(hass: HomeAssistant, entry: ConfigEntry) -> bool:
    unload_ok = await hass.config_entries.async_unload_platforms(entry, PLATFORMS)
    if unload_ok:
        hass.data[DOMAIN].pop(entry.entry_id, None)
    return unload_ok

