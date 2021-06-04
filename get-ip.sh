# Gets ip address
RESPONSE=$(curl https://api.ipify.org)
echo -e "ip=$RESPONSE" >> config.properties

# Sends ip address to cloud