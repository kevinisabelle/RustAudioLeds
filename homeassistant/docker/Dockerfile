# File: `docker/Dockerfile.ha`
FROM ghcr.io/home-assistant/home-assistant:stable
# Copy custom component (adjust path if your repo layout differs)
COPY custom_components/audioleds /config/custom_components/audioleds
# Optional: include any other resources (translations, services, etc.)
# COPY custom_components/audioleds/translations /config/custom_components/audioleds/translations
# Seed a minimal configuration.yaml (adjust as needed)
COPY docker/configuration.yaml /config/configuration.yaml
# Ensure readable permissions
RUN chmod -R 755 /config/custom_components/audioleds || true
