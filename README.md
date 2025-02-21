# SQL Query Executor

Este programa permite ejecutar consultas SQL sobre archivos que representan tablas en una carpeta específica.

## Formato de Uso

Para ejecutar el programa, se debe proporcionar:
1. La ruta a la carpeta que contiene los archivos de las tablas.
2. La consulta SQL a ejecutar.

### Ejemplo de Ejecución:
```sh
cargo run -- ruta/a/tablas "<consulta>"
```

### Ejemplo con una consulta SELECT:
```sh
cargo run -- ruta/a/tablas "SELECT * FROM table"
```

## Formato de Output

- **Consultas SELECT:**
  - El resultado se imprimirá en **STDOUT** en formato **CSV**.
  - Para verificar la salida, se puede redirigir a un archivo usando `>` y abrirlo con cualquier herramienta compatible con CSV.

  ```sh
  cargo run -- ruta/a/tablas "SELECT * FROM table" > output.csv
  ```

- **Otras consultas (INSERT, UPDATE, DELETE, etc.):**
  - No se imprimirá ninguna salida.
