"""Config flow for AudioLEDs integration."""
import logging

import voluptuous as vol
from homeassistant import config_entries
from homeassistant.const import CONF_HOST, CONF_PORT, CONF_VERIFY_SSL
from homeassistant.helpers.aiohttp_client import async_get_clientsession

from .const import DOMAIN

_LOGGER = logging.getLogger(__name__)

DATA_SCHEMA = vol.Schema(
    {
        vol.Required(CONF_HOST): str,
        vol.Required(CONF_PORT, default=443): int,
        vol.Optional(CONF_VERIFY_SSL, default=True): bool,
    }
)


class AudioLedsConfigFlow(config_entries.ConfigFlow, domain=DOMAIN):
    """Handle a config flow for AudioLEDs."""

    VERSION = 1

    async def async_step_user(self, user_input=None):
        """Handle the initial step."""
        errors = {}
        if user_input is not None:
            session = async_get_clientsession(self.hass)
            host = user_input[CONF_HOST]
            port = user_input[CONF_PORT]
            verify_ssl = user_input[CONF_VERIFY_SSL]

            # The API is always HTTPS. The 'verify_ssl' flag controls certificate validation.
            url = f"https://{host}:{port}/api/v1/info"

            try:
                # The `ssl` parameter handles certificate verification.
                # If False, it allows connection to a server with a self-signed certificate.
                async with session.get(url, timeout=10, ssl=verify_ssl) as response:
                    if response.status == 200:
                        await self.async_set_unique_id(host)
                        self._abort_if_unique_id_configured()
                        return self.async_create_entry(title=host, data=user_input)
                    else:
                        _LOGGER.error(
                            "Failed to connect to AudioLEDs device at %s: Status %s",
                            url,
                            response.status,
                        )
                        errors["base"] = "cannot_connect"
            except Exception as exc:
                _LOGGER.error(
                    "Failed to connect to AudioLEDs device at %s: %s", url, exc
                )
                errors["base"] = "cannot_connect"

        return self.async_show_form(
            step_id="user", data_schema=DATA_SCHEMA, errors=errors
        )
