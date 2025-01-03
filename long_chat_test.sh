#!/usr/bin/env bash

# A script to test a long conversation with the default prompt and 10 messages.

echo "== 1) Creating (or retrieving) a new user =="
USER_ID=$(
  curl -s -X POST -H "Content-Type: application/json" \
       -d '{"username": "longConvoUser"}' \
       http://localhost:3000/login \
  | jq '.user_id'
)

# If you want to see the entire JSON response, remove the '| jq .user_id'
echo "Got USER_ID=$USER_ID"
echo

# Sleep a bit to avoid hammering your server
sleep 2

echo "== 2) Checking the default prompt for this user =="
curl -s -X GET http://localhost:3000/prompt/$USER_ID | jq
echo
sleep 2

###############################################
# 10 MEANINGFUL MESSAGES
###############################################

echo "== 3) Sending first user message =="
curl -s -X POST -H "Content-Type: application/json" \
     -d '{
           "message": "Hello, I am feeling a bit anxious these days. My work is stable, but I keep questioning if I am on the right path. Could you help me figure out some steps to address this sense of uncertainty?",
           "user_id": '"$USER_ID"'
         }' \
     http://localhost:3000/chat \
| jq
echo
sleep 44

curl -s -X GET http://localhost:3000/summary/$USER_ID | jq
echo


echo "== 4) Sending second user message =="
curl -s -X POST -H "Content-Type: application/json" \
     -d '{
           "message": "Thank you for the suggestions. I tried journaling last night, and I realized I have a lot of negative thoughts about my own competence. It’s almost like I’m afraid to succeed, which is strange. Have you heard of something like that before?",
           "user_id": '"$USER_ID"'
         }' \
     http://localhost:3000/chat \
| jq
echo
sleep 44
curl -s -X GET http://localhost:3000/summary/$USER_ID | jq
echo
echo "== 5) Sending third user message =="
curl -s -X POST -H "Content-Type: application/json" \
     -d '{
           "message": "I see, so you think it’s partially about self-doubt. In that case, how do I challenge these inner beliefs? For example, if I believe I’m not smart enough, how can I push back on that mentally or practically?",
           "user_id": '"$USER_ID"'
         }' \
     http://localhost:3000/chat \
| jq
echo
sleep 44
curl -s -X GET http://localhost:3000/summary/$USER_ID | jq
echo
echo "== 6) Sending fourth user message =="
curl -s -X POST -H "Content-Type: application/json" \
     -d '{
           "message": "Yes, that’s an interesting perspective. I’d like to talk more about setting realistic goals and not beating myself up. However, I also wonder if I need a career change. I’m working in finance, but I keep dreaming about starting a small bakery.",
           "user_id": '"$USER_ID"'
         }' \
     http://localhost:3000/chat \
| jq
echo
sleep 44

echo "== 7) Sending fifth user message =="
curl -s -X POST -H "Content-Type: application/json" \
     -d '{
           "message": "Exactly, a bakery! I love to bake breads, pastries, and experiment with new recipes. It brings me joy. I feel so alive when I’m in the kitchen, but I worry I might fail if I go all in. How do I handle the fear of starting from scratch?",
           "user_id": '"$USER_ID"'
         }' \
     http://localhost:3000/chat \
| jq
echo
sleep 44

echo "== 8) Sending sixth user message =="
curl -s -X POST -H "Content-Type: application/json" \
     -d '{
           "message": "That’s a good point about planning. Maybe I can try running a small pop-up bakery on weekends first. By the way, I also want to mention that finances are tight, so I can’t afford to just quit my job. This is a real juggling act.",
           "user_id": '"$USER_ID"'
         }' \
     http://localhost:3000/chat \
| jq
echo
sleep 44

echo "== 9) Sending seventh user message =="
curl -s -X POST -H "Content-Type: application/json" \
     -d '{
           "message": "Well, I guess I should also talk about my personal life. Sometimes, the stress of my job and my side passions leaves me little time for my friends. I’m feeling guilty about being distant. How do I manage that guilt while pursuing my own goals?",
           "user_id": '"$USER_ID"'
         }' \
     http://localhost:3000/chat \
| jq
echo
sleep 44

echo "== 10) Sending eighth user message =="
curl -s -X POST -H "Content-Type: application/json" \
     -d '{
           "message": "True, so maybe I can be more proactive about scheduling friend time. Let’s pivot for a moment and talk about something deeper: I often catch myself thinking, ‘I’m not interesting enough to be around,’ which pushes me to isolate. How can I start believing I’m worth social engagement?",
           "user_id": '"$USER_ID"'
         }' \
     http://localhost:3000/chat \
| jq
echo
sleep 44

echo "== 11) Sending ninth user message =="
curl -s -X POST -H "Content-Type: application/json" \
     -d '{
           "message": "That’s helpful, thanks. I’ll try to notice and reframe those negative thoughts. Do you recommend any practical exercises, like writing down affirmations or trying out group activities, to reinforce positive beliefs about myself?",
           "user_id": '"$USER_ID"'
         }' \
     http://localhost:3000/chat \
| jq
echo
sleep 44

echo "== 12) Sending tenth user message =="
curl -s -X POST -H "Content-Type: application/json" \
     -d '{
           "message": "Yes, I see how that might help. I’ll try some of those exercises this week. Honestly, this has been really enlightening so far. Could you share any final thoughts on balancing personal growth, career dreams, and healthy relationships?",
           "user_id": '"$USER_ID"'
         }' \
     http://localhost:3000/chat \
| jq
echo
sleep 44

# Finally, let's check the summary of this entire conversation
echo "== 13) Fetching final summary from the database =="
curl -s -X GET http://localhost:3000/summary/$USER_ID | jq
echo
echo "== Done! Check above for the summarized conversation content. =="

