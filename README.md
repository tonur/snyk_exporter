# Snyk-Exporter
Simple Tenable exporter to get Open-Telemetry metrics of Snyk Vulnerabilities.

## Usage
Run the Docker image with the following variables:
``` yaml
snyk.api-token: 
```
``` sh
docker run -p 8080:<your_port> -e snyk.api-token=your_token tonur/snyk-exporter
```