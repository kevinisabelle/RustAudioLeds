"""Data update coordinator for the AudioLEDs integration."""
from datetime import timedelta
import logging

from homeassistant.helpers.aiohttp_client import async_get_clientsession
from homeassistant.helpers.update_coordinator import DataUpdateCoordinator, UpdateFailed

from .const import DOMAIN

_LOGGER = logging.getLogger(__name__)


class AudioLedsDataUpdateCoordinator(DataUpdateCoordinator):
    """Class to manage fetching AudioLEDs data from the device."""

    def __init__(self, hass, *, host: str, port: int):
        """Initialize."""
        self.api_url = f"http://{host}:{port}/api/v1"
        super().__init__(
            hass,
            _LOGGER,
            name=DOMAIN,
            update_interval=timedelta(seconds=10),
        )

    async def _async_update_data(self):
        """Fetch data from API endpoint."""
        session = async_get_clientsession(self.hass)
        try:
            async with session.get(f"{self.api_url}/info", timeout=10) as response:
                if response.status != 200:
                    raise UpdateFailed(f"Error communicating with API: {response.status}")
                info_data = await response.json()

            async with session.get(f"{self.api_url}/presets", timeout=10) as response:
                if response.status != 200:
                    raise UpdateFailed(f"Error communicating with API: {response.status}")
                presets_data = await response.json()

            return {"info": info_data, "presets": presets_data}
        except Exception as exc:
            raise UpdateFailed(f"Error communicating with API: {exc}") from exc

