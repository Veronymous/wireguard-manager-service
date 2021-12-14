#!/bin/sh

export WIREGUARD_PATH=/etc/wireguard
export PRIVATE_KEY_PATH=$WIREGUARD_PATH/private.key
export PUBLIC_KEY_PATH=$WIREGUARD_PATH/public.key

mkdir -p /etc/wireguard/

cd $WIREGUARD_PATH && umask 077

# Generate the keys
if [ ! -f $PRIVATE_KEY_PATH ]; then
  echo "Generating a new key pair..."
  wg genkey >> $PRIVATE_KEY_PATH

  # Remove old public key
  rm -rf $PUBLIC_KEY_PATH

  # shellcheck disable=SC2002
  cat "$PRIVATE_KEY_PATH" | wg pubkey >> $PUBLIC_KEY_PATH
else
  if [ ! -f $PUBLIC_KEY_PATH ]; then
      # shellcheck disable=SC2002
      cat "$PRIVATE_KEY_PATH" | wg pubkey >> $PUBLIC_KEY_PATH
  fi
fi