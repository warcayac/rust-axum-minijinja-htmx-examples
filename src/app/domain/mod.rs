/*
1. Domain Layer (Core Business Logic)
---------------------------------------
- Purpose: The Domain Layer forms the core of the application, containing essential business logic, rules, 
    and entities. It represents the fundamental behaviors and processes that define the application’s 
    primary purpose.
- Contents: This layer includes business entities, value objects, and domain services that enforce critical 
    rules, validations, and calculations.
- Independence: The Domain Layer is isolated from all other layers and external dependencies, remaining stable 
    even when external systems, such as databases or APIs, are updated or replaced.
*/
pub mod entities;