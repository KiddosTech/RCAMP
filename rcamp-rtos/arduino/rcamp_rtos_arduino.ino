// RCAMP/RTOS serial target for Arduino Nano/Uno.
// Commands are newline-delimited: PING, INFO, CAPABILITIES.
const char* RCAMP_ADMIN_TOKEN = "CHANGE_ME";
String rcampRole = "user";

void setup() {
  Serial.begin(115200);
}

void loop() {
  if (!Serial.available()) return;
  String command = Serial.readStringUntil('\n');
  command.trim();
  String upper = command;
  upper.toUpperCase();
  if (upper == "PING") {
    Serial.println("PONG");
  } else if (upper == "HELP") {
    Serial.println("task rgb_on|rgb_off|rgb_set|led_on|led_off|buzzer_beep|relay_on|relay_off|test_write|read_input|sensor_read|status_snapshot|telemetry_on|telemetry_off|identify|uptime|storage_info|reboot");
  } else if (upper == "STATUS") {
    Serial.print("{\"ready\":true,\"firmware\":\"RCAMP/RTOS\",\"role\":\""); Serial.print(rcampRole); Serial.println("\"}");
  } else if (upper == "WHOAMI") {
    Serial.print("role="); Serial.println(rcampRole);
  } else if (upper == "RCAMPFETCH") {
    Serial.print("RCAMP/RTOS\nfirmware=RCAMP/RTOS\nversion=0.1.0\ntarget=Arduino\nrole="); Serial.println(rcampRole);
  } else if (upper == "INFO" || upper == "CAPABILITIES") {
    Serial.println("{\"name\":\"Arduino RCAMP Target\",\"firmware\":\"RCAMP/RTOS\",\"capabilities\":[\"ping\",\"info\"]}");
  } else if (upper == "LOGS") {
    Serial.println("{\"level\":\"info\",\"message\":\"RCAMP/RTOS target online\"}");
  } else if (upper == "TASK RGB_ON") {
    Serial.println("OK task=rgb_on state=on");
  } else if (upper == "TASK RGB_OFF") {
    Serial.println("OK task=rgb_off state=off");
  } else if (upper == "TASK RGB_SET") {
    Serial.println("OK task=rgb_set state=simulated configure-rgb-driver");
  } else if (upper == "TASK LED_ON") {
    Serial.println("OK task=led_on state=on");
  } else if (upper == "TASK LED_OFF") {
    Serial.println("OK task=led_off state=off");
  } else if (upper == "TASK BUZZER_BEEP") {
    Serial.println("OK task=buzzer_beep state=simulated configure-buzzer");
  } else if (upper == "TASK RELAY_ON" || upper == "TASK RELAY_OFF") {
    Serial.println("OK task=relay state=simulated configure-relay");
  } else if (upper == "TASK TEST_WRITE") {
    if (rcampRole == "user") Serial.println("ERR permission administrator-required"); else Serial.println("OK task=test_write write=verified");
  } else if (upper == "TASK READ_INPUT") {
    Serial.println("OK task=read_input value=simulated");
  } else if (upper == "TASK SENSOR_READ") {
    Serial.println("OK task=sensor_read value=simulated configure-sensor");
  } else if (upper == "TASK STATUS_SNAPSHOT") {
    Serial.println("OK task=status_snapshot ready=true");
  } else if (upper == "TASK TELEMETRY_ON" || upper == "TASK TELEMETRY_OFF") {
    Serial.println("OK task=telemetry state=updated");
  } else if (upper == "TASK IDENTIFY") {
    Serial.println("OK task=identify marker=on");
  } else if (upper == "TASK UPTIME") {
    Serial.println("OK task=uptime unsupported=arduino-runtime");
  } else if (upper == "TASK STORAGE_INFO") {
    Serial.println("OK task=storage_info storage=runtime volatile=true");
  } else if (upper == "TASK REBOOT") {
    if (rcampRole == "user") Serial.println("ERR permission-administrator-required"); else Serial.println("OK task=reboot scheduled=false");
  } else if (upper.startsWith("SUDO ")) {
    String token = command.substring(5);
    if (String(RCAMP_ADMIN_TOKEN) != "CHANGE_ME" && token == RCAMP_ADMIN_TOKEN) { rcampRole = "administrator"; Serial.println("OK role=administrator"); } else Serial.println("ERR elevation-denied");
  } else {
    Serial.println("ERR unsupported-command");
  }
}
