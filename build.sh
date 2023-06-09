#!/bin/bash

# Verificar si rustc está instalado
if ! command -v rustc &> /dev/null
then
    echo "rustc no se encontró. Por favor, instálalo antes de correr este script."
    exit 1
fi

# Compilar server.rs
rustc server.rs -o server

# Verificar si la compilación de server.rs fue exitosa
if [ $? -eq 0 ]; then
    echo "server.rs se compiló correctamente."
else
    echo "Hubo un error al compilar server.rs."
    exit 1
fi

# Compilar cliente.rs
rustc cliente.rs -o cliente

# Verificar si la compilación de cliente.rs fue exitosa
if [ $? -eq 0 ]; then
    echo "cliente.rs se compiló correctamente."
else
    echo "Hubo un error al compilar cliente.rs."
    exit 1
fi

echo "Ambos archivos se compilaron con éxito."
exit 0

