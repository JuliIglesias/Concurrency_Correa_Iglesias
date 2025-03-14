# Concurrency Programming

Students:
- Correa Ignacio
- Iglesias Julieta


## TP1 ~ HTTP Server

1. ¿Qué sucede con dos requests simultáneas que tardan en procesarse?

Al mandar dos requests simultáneamente (la primera con un n muy grande y la segunda con uno muy chico), 
observamos que la segunda, a pesar de ser mucho más pequeña, tuvo que esperar a que se finalice el
cálculo de la primera para devolver su respuesta.

![img.png](img.png)

2. ¿Por qué se observa este comportamiento?

Esto se debe a que el servidor es single-threaded y no puede procesar ambas request al mismo tiempo. :(

3. ¿Cómo solucionar usando solo librerías estándar de Rust?


