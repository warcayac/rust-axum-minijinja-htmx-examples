/*
4. Presentation Layer (UI/Endpoints)
------------------------------------
- Purpose: The Presentation Layer serves as the system’s entry point, such as APIs in a backend service or user 
    interfaces in a frontend. It gathers and interprets input from users or external systems and directs it to 
    the Application Layer.
- Contents: This layer includes controllers, view models, UI components, and other elements that handle 
    interactions with users or client systems.
- Interaction with Application Layer: The Presentation Layer communicates exclusively with the Application Layer, 
    which in turn coordinates with the Domain and Infrastructure Layers to fulfill requests. This separation 
    ensures clear boundaries and maintains the integrity of each layer.
*/
pub mod views;
pub mod api;
mod helpers;
