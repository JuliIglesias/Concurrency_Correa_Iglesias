## TP2 ~ MiniGrep

1. ¿Cómo se comparan los tiempos de ejecución entre la implementación secuencial y la concurrente?

Luego de probar el programa con distintas cantidades de textos, encontramos que la concurrencia logra
una diferencia sustancial contra la búsqueda y el análisis secuencial de los textos. Comparando el
concurrente con el c-chunk, se han visto resultados más consistentes con el concurrente: Se debe a que
el c-chunk se ve fuertemente afectado por el tamaño de los chunks, los cuales dependen del tamaño del
texto. Este logra ser eficiente una vez que se encuentra un "punto medio" en el tamaño de los chunks.

2. Al reducir el tamaño de los segmentos (chunks), ¿qué patrón se observa en los tiempos de ejecución?
¿A qué se debe esto?

Reduciendo el tamaño de los chunks afecta directamente el tiempo de procesamiento, ya que, en esta
forma de procesamiento, se genera 1 thread por cada chunk. Al ser el chunk de menor tamaño, en más
segmentos se termina separando el texto a analizar, generando una cola de threads enorme que se espera
a que se liberen los recursos de la computadora.
