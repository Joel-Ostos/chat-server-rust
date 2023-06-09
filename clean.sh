# Se elimina el ejecutable server y cliente
rm -r server cliente
# Se verifica si la compilación de server.rs fue exitosa
if [ $? -eq 0 ]; then
    echo "server se eliminó correctamente."
else
    echo "Hubo un error al eliminar el server."
    exit 1
fi

# Se verifica si la eliminación de cliente fue exitosa
if [ $? -eq 0 ]; then
    echo "cliente.rs se eliminó correctamente."
else
    echo "Hubo un error al eliminar cliente."
    exit 1
fi

echo "Ambos archivos se eliminaron con éxito."
exit 0

