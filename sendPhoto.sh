#!/bin/bash

url="https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/sendPhoto"
curl -X POST "$url" -F chat_id="$1" -F photo="@$2" -F disable_notification=true
