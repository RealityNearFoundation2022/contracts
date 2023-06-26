#!/bin/bash

# Solicita los parámetros
read -p 'Ingresa el valor de TOKEN_SALE: ' TOKEN_SALE
read -p 'Ingresa el valor de CONTRACT: ' CONTRACT
read -p 'Ingresa el valor de AMOUNT (en unidades sin decimales): ' AMOUNT

# Agrega ocho ceros al valor de AMOUNT
AMOUNT="${AMOUNT}00000000"

# Solicita la fecha de inicio (START) y la duración (DURATION)
read -p 'Ingresa la fecha de inicio en formato YYYY-MM-DD HH:MM:SS: ' START
read -p 'Ingresa la duración en días: ' DURATION

# Convierte las fechas a formato de tiempo de UNIX en nanosegundos
START=$(date -d"$START" +%s)000000000
DURATION=$(echo $DURATION*24*60*60*1000000000 | bc)

# Llama al contrato
near call $CONTRACT --accountId $CONTRACT new '{"owner":"'$CONTRACT'", "token":"'$TOKEN_SALE'","amount":"'$AMOUNT'","start":"'$START'", "duration":"'$DURATION'"}'

