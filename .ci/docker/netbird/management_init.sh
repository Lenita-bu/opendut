#!/bin/bash

export OTEL_EXPORTER_OTEL_ENDPOINT="http://192.168.32.220:4318"
export OTEL_SERVICE_NAME="netbird-shell"
export OTEL_SH_LIB_PATH="opentelemetry-shell/library"
. "${OTEL_SH_LIB_PATH}/otel_traces.sh"


ls -la /usr/local/share/ca-certificates/
/usr/sbin/update-ca-certificates
source /netbird-api-functions.sh


#dummy_trace() {
 # echo "dummy"
#}

#otel_trace_start_parent_span dummy_trace
#exit_code=$?
#echo "Exit code: $exit_code"

otel_trace_start_parent_span "management_init.sh"

otel_trace_add_child_span wait_for_url "$KEYCLOAK_URL" 600 5 || exit 1
echo "Keycloak available"

otel_trace_add_child_span wait_for_url "$KEYCLOAK_REALM_URL" 600 5 || exit 1
echo "Keycloak realm available: $KEYCLOAK_REALM_URL"

otel_trace_add_child_span wait_for_keycloak_client_auth_successful 600 5 || exit 1
echo "Keycloak ready"

otel_trace_add_child_span wait_for_keycloak_user__in_realm_netbird "netbird" 600 5 || exit 1
otel_trace_add_child_span wait_for_keycloak_client__in_realm_netbird "netbird-backend" 600 5 || exit 1

/go/bin/netbird-mgmt management --port 80 --log-file console --disable-anonymous-metrics=true --single-account-mode-domain=netbird.opendut.local --dns-domain=netbird.opendut.local
