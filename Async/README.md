# TP7 - Comparación de Concurrencia: Threads vs Async (Tokio)

Este trabajo práctico compara dos enfoques de programación concurrente en Rust: el modelo tradicional basado en threads y el modelo basado en async programming (Tokio). Se simulan tareas de I/O y de cálculo intensivo (Pi con la serie de Leibniz) usando ambos enfoques, midiendo y analizando diferencias de performance.

## Ejecución

Compilar en modo release para obtener mediciones realistas:

```
cargo build --release
```

### Argumentos principales

- `--mode [thread|async]`   : Elige el modelo de concurrencia.
- `--task [io|pi]`           : Elige el tipo de tarea a simular.
- `--tasks N`                : Número de tareas concurrentes (default: 10).
- `--terms N`                : Cantidad de términos para el cálculo de Pi (solo para `pi`).
- `--millis N`               : Milisegundos de espera por tarea (solo para `io`).

### Ejemplos

Simular 100 tareas de I/O con threads, cada una esperando 200ms:
```
cargo run --release -- --mode thread --task io --tasks 100 --millis 200
```

Simular 1000 tareas de I/O con async:
```
cargo run --release -- --mode async --task io --tasks 1000 --millis 100
```

Calcular Pi con 8 tareas y 1 millón de términos usando threads:
```
cargo run --release -- --mode thread --task pi --tasks 8 --terms 1000000
```

Calcular Pi con async:
```
cargo run --release -- --mode async --task pi --tasks 8 --terms 1000000
```

## Estructura del código

- `src/main.rs`   : Parseo de argumentos y orquestación.
- `src/io_tasks.rs`: Simulación de tareas de I/O (thread y async).
- `src/pi_calc.rs` : Cálculo concurrente de Pi (thread y async).
- `src/lib.rs`     : Módulos y helpers generales.

## Análisis sugerido

- Comparar tiempos de ejecución y uso de CPU para ambos modelos y tareas.
- Probar con distintos valores de `--tasks` y `--terms`.
- Analizar cuándo y por qué async es más eficiente que threads para I/O.
- Analizar el comportamiento en tareas CPU-bound (cálculo de Pi).

## Requisitos

- Rust 2021
- [Tokio](https://tokio.rs/) (para async)
- [clap](https://docs.rs/clap/) (para argumentos)

---

Trabajo Práctico 7 - Programación Concurrente

