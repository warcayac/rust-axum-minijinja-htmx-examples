/*
3. Infrastructure Layer (External Systems)
------------------------------------------
- Purpose: The Infrastructure Layer manages interactions with external systems, including databases, file storage,
    logging, and third-party services. It provides the implementations needed by the Domain and Application Layers 
    for these external connections.
- Contents: This layer includes repositories, logging mechanisms, data access implementations (e.g., Entity 
    Framework or Dapper), and third-party integrations.
- Interaction with Application Layer: The Infrastructure Layer fulfills interfaces and abstractions defined by 
    the Domain and Application Layers. It does not contain business logic but instead provides necessary support 
    for storing, retrieving, and processing data through external services.
*/