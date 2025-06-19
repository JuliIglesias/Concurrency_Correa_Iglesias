# TP7 - Comparación de Concurrencia: Threads vs Async (Tokio)

Este trabajo práctico compara dos enfoques de programación concurrente en Rust: el modelo tradicional basado en threads y el modelo basado en async programming (Tokio). Se simulan tareas de I/O y de cálculo intensivo (Pi con la serie de Leibniz) usando ambos enfoques, midiendo y analizando diferencias de performance.

## Ejecución

```
    cargo run [thread|async] [io|pi] --n N --t N --m N
```

### Argumentos principales

- `[thread|async]`    : Elige el modelo de concurrencia.
- `[io|pi]`           : Elige el tipo de tarea a simular.
- `--n N`                : Número de tareas concurrentes (default: 10).
- `--t N`                : Cantidad de términos para el cálculo de Pi (solo para `pi`).
- `--m N`               : Milisegundos de espera por tarea (solo para `io`).

### Ejemplos

Simular 100 tareas de I/O con threads, cada una esperando 200ms:
```
    cargo run thread io -n 100 -m 200
```

Simular 1000 tareas de I/O con async:
```
    cargo run async io -n 1000 -m 100
```

Calcular Pi con 8 tareas y 1 millón de términos usando threads:
```
    cargo run thread pi -n 8 -t 1000000
```

Calcular Pi con async:
```
    cargo run async pi -n 8 -t 1000000
```

## Estructura del código

- `src/main.rs`   : Parseo de argumentos y orquestación.
- `src/io_tasks.rs`: Simulación de tareas de I/O (thread y async).
- `src/pi_calc.rs` : Cálculo concurrente de Pi (thread y async).
- `src/lib.rs`     : Módulos y helpers generales.

## Análisis sugerido

- Comparar tiempos de ejecución y uso de CPU para ambos modelos y tareas.
- Probar con distintos valores de `--n` y `--t`.
- Analizar cuándo y por qué async es más eficiente que threads para I/O.
- Analizar el comportamiento en tareas CPU-bound (cálculo de Pi).

## Requisitos

- Rust 2021
- [Tokio](https://tokio.rs/) (para async)

---
