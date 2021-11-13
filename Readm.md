# Propuesta de solución en Rust para Hackathon Ceiba 2021

## Descripción de la solución
Para la solución se ha decidido implementar un controlador web tipo get, el cual hace uso de un
servicio que se apropia de la tranformación de la respuesta entregada por la api.

Dentro del servicio podemos encontrar manejo de caché y de resiliencia, con el fin de procesar de una forma óptima la entrada y resolver en una salida mejorada.

## Drivers que orientan la toma de decisiones para la solución propuesta

-Velocidad de procesamiento/optimización de memoria: con Rush nos apropiamos de su velocidad de procesamiento al no hacer uso ni tener que levantar una VM y/o garbage collector, además de la reducción en tiempos
de latencia al manipular cached -> temporal en archivos locales.

-Modificabilidad: El costo de realizar cambios será minimo al distribuir el microservicio en modulos, separando la capa lógica de la api.

## Diagrama de solución

![Diagrama componentes!](./design.drawio.svg "Diagrama de Solución")

| Elemento | Descripción   |
| :------- | :------------ |
| main       | Arrancar el servidor y configurar las propiedades de los endpoind(path, content, etc)|
| controller | Exponer los endpoints - ya sean de health o  consumo de api                          |
| service    | desarrollo de la logica y procesamiento de las entradas - retorno de salida procesada|
| response   | estructura que conservará las mismas propiedades de la entrada a modo de conversión  |

