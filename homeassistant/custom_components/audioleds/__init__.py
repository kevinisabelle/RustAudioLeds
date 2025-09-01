from __future__ import annotations

import logging
from typing import Any

from homeassistant.config_entries import ConfigEntry
from homeassistant.core import HomeAssistant, ServiceCall
from homeassistant.helpers.typing import ConfigType
from homeassistant.helpers import aiohttp_client
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
    SERVICE_SET_COLORS,
    ATTR_PRESET_ID,
)
from .coordinator import AudioLedsDataUpdateCoordinator

_LOGGER = logging.getLogger(__name__)

SEND_COMMAND_SCHEMA = vol.Schema({vol.Required("data"): dict})
CREATE_PRESET_SCHEMA = vol.Schema({vol.Required("data"): dict})
ACTIVATE_PRESET_SCHEMA = vol.Schema({vol.Required(ATTR_PRESET_ID): vol.Coerce(int)})
DELETE_PRESET_SCHEMA = vol.Schema({vol.Required(ATTR_PRESET_ID): vol.Coerce(int)})

# Color validator: either dict with r,g,b or list/tuple of 3 ints 0-255
_COLOR_LIST = vol.All([vol.All(int, vol.Range(min=0, max=255))], vol.Length(min=3, max=3))
_COLOR_DICT = vol.Schema({vol.Required("r"): vol.All(int, vol.Range(0, 255)), vol.Required("g"): vol.All(int, vol.Range(0, 255)), vol.Required("b"): vol.All(int, vol.Range(0, 255))})
SET_COLORS_SCHEMA = vol.Schema(
    {
        vol.Optional("color1"): vol.Any(_COLOR_DICT, _COLOR_LIST),
        vol.Optional("color2"): vol.Any(_COLOR_DICT, _COLOR_LIST),
        vol.Optional("color3"): vol.Any(_COLOR_DICT, _COLOR_LIST),
    }
)


async def async_setup(hass: HomeAssistant, config: ConfigType) -> bool:  # noqa: D401
    """Set up via config entries only."""
    return True


async def async_setup_entry(hass: HomeAssistant, entry: ConfigEntry) -> bool:
    hass.data.setdefault(DOMAIN, {})

    conf = entry.data
    session = aiohttp_client.async_get_clientsession(hass)
    api = AudioLedsApi(
        session,
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

        async def _handle_set_colors(call: ServiceCall) -> None:
            payload: dict[str, Any] = {}
            for key in ("color1", "color2", "color3"):
                if key in call.data:
                    val = call.data[key]
                    if isinstance(val, (list, tuple)) and len(val) == 3:
                        r, g, b = (int(v) for v in val)
                        payload[key] = {"r": r, "g": g, "b": b}
                    elif isinstance(val, dict):
                        r, g, b = int(val["r"]), int(val["g"]), int(val["b"])
                        payload[key] = {"r": r, "g": g, "b": b}
            if not payload:
                _LOGGER.debug("set_colors called with no colors; skipping")
                return
            _LOGGER.debug("Service set_colors: %s", payload)
            await api.async_post_command(payload)
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
        hass.services.async_register(
            DOMAIN, SERVICE_SET_COLORS, _handle_set_colors, schema=SET_COLORS_SCHEMA
        )

    await hass.config_entries.async_forward_entry_setups(entry, PLATFORMS)
    return True


async def async_unload_entry(hass: HomeAssistant, entry: ConfigEntry) -> bool:
    unload_ok = await hass.config_entries.async_unload_platforms(entry, PLATFORMS)
    if unload_ok:
        hass.data[DOMAIN].pop(entry.entry_id, None)
    return unload_ok
