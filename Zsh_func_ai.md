ai() {
  local prompt="$*"

  curl -s -X POST http://192.168.xx.xx:8080/v1/chat/completions \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer no-key" \
    -d "$(printf '{
      "model": "nemotron",
      "temperature": 0.4,
      "top_p": 0.9,
      "max_tokens": 512,
      "messages": [
        {
          "role": "system",
          "content": "You are a precise, truthful assistant. If you do not know, say so briefly."
        },
        {
          "role": "user",
          "content": "%s"
        }
      ]
    }' "$(printf '%s' "$prompt" | sed 's/"/\\"/g')" )" \
  | jq -r '.choices[0].message.content'
}
