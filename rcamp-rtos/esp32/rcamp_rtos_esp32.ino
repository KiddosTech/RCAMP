#include <WiFi.h>

const char* RCAMP_WIFI_SSID = "CHANGE_ME";
const char* RCAMP_WIFI_PASSWORD = "CHANGE_ME";
constexpr uint16_t RCAMP_PORT = 8080;
constexpr uint8_t RCAMP_RGB_PIN = 2;
// Set this during provisioning. A CHANGE_ME token never grants elevation.
const char* RCAMP_ADMIN_TOKEN = "CHANGE_ME";
WiFiServer rcampServer(RCAMP_PORT);
String rcampRole = "user";

void sendInfo(WiFiClient& client) {
  client.println("{\"name\":\"ESP32 RCAMP Target\",\"firmware\":\"RCAMP/RTOS\",\"capabilities\":[\"ping\",\"info\"]}");
}

void handleCommand(WiFiClient& client, String command) {
  command.trim();
  String upper = command; upper.toUpperCase();
  if (upper == "PING") client.println("PONG");
  else if (upper == "HELP") client.println("task rgb_on|rgb_off|rgb_set|led_on|led_off|buzzer_beep|relay_on|relay_off|test_write|read_input|sensor_read|status_snapshot|telemetry_on|telemetry_off|identify|uptime|storage_info|reboot");
  else if (upper == "STATUS") client.printf("{\"ready\":true,\"role\":\"%s\",\"uptime_ms\":%lu,\"heap\":%u}\n", rcampRole.c_str(), millis(), ESP.getFreeHeap());
  else if (upper == "INFO" || upper == "CAPABILITIES") sendInfo(client);
  else if (upper == "WHOAMI") client.printf("role=%s\n", rcampRole.c_str());
  else if (upper == "RCAMPFETCH") client.printf("RCAMP/RTOS\nfirmware=RCAMP/RTOS\nversion=0.1.0\ntarget=ESP32\nrole=%s\nuptime_ms=%lu\nheap=%u\n", rcampRole.c_str(), millis(), ESP.getFreeHeap());
  else if (upper == "LOGS") client.println("{\"level\":\"info\",\"message\":\"RCAMP/RTOS target online\"}");
  else if (upper == "TASK RGB_ON") { digitalWrite(RCAMP_RGB_PIN, HIGH); client.println("OK task=rgb_on state=on"); }
  else if (upper == "TASK RGB_OFF") { digitalWrite(RCAMP_RGB_PIN, LOW); client.println("OK task=rgb_off state=off"); }
  else if (upper == "TASK RGB_SET") client.println("OK task=rgb_set state=simulated configure-rgb-driver");
  else if (upper == "TASK LED_ON") { digitalWrite(RCAMP_RGB_PIN, HIGH); client.println("OK task=led_on state=on"); }
  else if (upper == "TASK LED_OFF") { digitalWrite(RCAMP_RGB_PIN, LOW); client.println("OK task=led_off state=off"); }
  else if (upper == "TASK BUZZER_BEEP") client.println("OK task=buzzer_beep state=simulated configure-buzzer");
  else if (upper == "TASK RELAY_ON") client.println("OK task=relay_on state=simulated configure-relay");
  else if (upper == "TASK RELAY_OFF") client.println("OK task=relay_off state=simulated configure-relay");
  else if (upper == "TASK TEST_WRITE") { if (rcampRole == "user") client.println("ERR permission administrator-required"); else client.println("OK task=test_write write=verified"); }
  else if (upper == "TASK READ_INPUT") client.println("OK task=read_input value=simulated");
  else if (upper == "TASK SENSOR_READ") client.println("OK task=sensor_read value=simulated configure-sensor");
  else if (upper == "TASK STATUS_SNAPSHOT") client.printf("OK task=status_snapshot ready=true heap=%u uptime_ms=%lu\n", ESP.getFreeHeap(), millis());
  else if (upper == "TASK TELEMETRY_ON") client.println("OK task=telemetry_on state=on");
  else if (upper == "TASK TELEMETRY_OFF") client.println("OK task=telemetry_off state=off");
  else if (upper == "TASK IDENTIFY") { digitalWrite(RCAMP_RGB_PIN, HIGH); client.println("OK task=identify marker=on"); }
  else if (upper == "TASK UPTIME") client.printf("OK task=uptime ms=%lu\n", millis());
  else if (upper == "TASK STORAGE_INFO") client.println("OK task=storage_info storage=runtime volatile=true");
  else if (upper == "TASK REBOOT") { if (rcampRole == "user") client.println("ERR permission-administrator-required"); else client.println("OK task=reboot scheduled=false"); }
  else if (upper.startsWith("SUDO ")) { String token = command.substring(5); if (String(RCAMP_ADMIN_TOKEN) != "CHANGE_ME" && token == RCAMP_ADMIN_TOKEN) { rcampRole = "administrator"; client.println("OK role=administrator"); } else client.println("ERR elevation-denied"); }
  else client.println("ERR unsupported-command");
  client.flush();
}

void rcampNetworkTask(void*) {
  for (;;) {
    WiFiClient client = rcampServer.available();
    if (client) {
      client.setNoDelay(true);
      client.setTimeout(2);
      handleCommand(client, client.readStringUntil('\n'));
      client.stop();
    }
    vTaskDelay(pdMS_TO_TICKS(1));
  }
}

void setup() {
  Serial.begin(115200);
  pinMode(RCAMP_RGB_PIN, OUTPUT);
  WiFi.begin(RCAMP_WIFI_SSID, RCAMP_WIFI_PASSWORD);
  while (WiFi.status() != WL_CONNECTED) delay(250);
  rcampServer.begin();
  Serial.printf("RCAMP/RTOS ready at %s:%u\n", WiFi.localIP().toString().c_str(), RCAMP_PORT);
  xTaskCreatePinnedToCore(rcampNetworkTask, "rcamp_network", 4096, nullptr, 1, nullptr, 1);
}

void loop() { delay(1000); }
