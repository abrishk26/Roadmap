# Personal Blog API

This is a simple Personal Blog API built with **Express**, **EJS**, and **Node.js**. It allows you to manage blog articles, including creating, reading, updating, and deleting articles. The application uses a JSON file (`articles.json`) to store article data.

Solution for the [personal-blog](https://roadmap.sh/projects/personal-blog) challenge from [roadmap.sh](https://roadmap.sh).

## Features

- **View all articles**: Display a list of all blog articles.
- **Create a new article**: Add a new article with a title, date, and content.
- **Update an article**: Modify the title, date, or content of an existing article.
- **Delete an article**: Remove an article from the list.
- **Admin authentication**: Secure the admin panel with email and password authentication.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Node.js](https://nodejs.org/) (v14 or higher)
- [npm](https://www.npmjs.com/) (comes with Node.js)
- [nodemon](https://nodemon.io/) (install globally for development)

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/abrishk26/personal-blog.git
   cd personal-blog
   ```

2. **Install dependencies**:
   ```bash
   npm install
   ```

3. **Install nodemon globally** (if not already installed):
   ```bash
   npm install -g nodemon
   ```

4. **Set up the `articles.json` file**:
   - The application will automatically create an `articles.json` file in the root directory when you start the server for the first time.

## Running the Application

1. **Start the server**:
   ```bash
   npm run start
   ```

2. **Access the application**:
   - Open your browser and go to `http://localhost:3000`.
   - Use the following credentials to log in to the admin panel:
     - **Email**: `admin@example.com`
     - **Password**: `password123`

## Available Scripts

- **`npm run start`**: Start the server using `nodemon` for development.

## API Endpoints

| Method | Endpoint          | Description                          |
|--------|-------------------|--------------------------------------|
| GET    | `/`               | Home page (list of articles)         |
| GET    | `/login`          | Admin login page                     |
| POST   | `/login`          | Admin login form submission          |
| GET    | `/new`            | Form to create a new article         |
| POST   | `/new`            | Create a new article                 |
| GET    | `/update/:id`     | Form to update an existing article   |
| POST   | `/update/:id`     | Update an existing article           |
| GET    | `/delete/:id`     | Delete an article                    |

## Dependencies

- **express**: Fast, unopinionated, minimalist web framework for Node.js.
- **ejs**: Embedded JavaScript templating for rendering dynamic HTML.
- **express-session**: Session management for Express.
- **nodemon**: Utility to automatically restart the server during development.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.
