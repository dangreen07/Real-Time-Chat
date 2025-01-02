# Real-Time-Chat
This is a simple chat application that uses the WebSocket protocol to communicate between the client and the server.

## Features
- User authentication
- Real-time chat
- Contact management
- Theme switching

## Prerequisites
- Docker
- Docker Compose

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/dangreen07/Real-Time-Chat.git
   ```  
2. Change the working directory to the cloned repository:
   ```bash
   cd Real-Time-Chat
   ```  
3. Setup the environment variables:
   The following environment variables are required:
   - `SERVER_URL`: The URL of the server. This is used to connect to the server.
   - `WS_SERVER_URL`: The URL of the WebSocket server. This is used to establish a WebSocket connection with the server.  
   - `DATABASE_URL`: The URL of the database. This is used to store user data.
   These environment variables can be set in a `.env` file in the root directory of the project.
   Alternatively, you can set them as environment variables in the terminal before running the docker compose file.  
4. Set up the database:
   To setup the database, go into the 'backend/migrations' directory and run the SQL commands named 'up.sql' in the order they are listed. You don't need to run the 'up.sql' file from the '00000000000000_diesel_initial_setup' directory.  
5. Run the following command to pull and run the docker compose file:
   ```bash
   docker compose up -d --pull always
   ```