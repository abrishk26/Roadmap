# Weather App

Sample solution for the [weather-app](https://roadmap.sh/projects/weather-api-wrapper-service) challenge from [roadmap.sh](https://roadmap.sh).

This is a weather app that allows users to view current weather conditions and forecasts for any location using the Visual Crossing's API.


## Installation

1. Clone the repository:
   ```sh
   git clone <repository-url>
   cd task_tracker
   ```

## Usage

1. Navigate to the project folder

```sh
cd weather-app
```

2. Install the required dependencies

   ```bash
   npm install
   ```
3. Create a .env file in root folder of the project and add your Visual Crossing's API_KEY and your Redis sever REDIS_URL.

   ```bash
   touch .env
   API_KEY="your_api_key"
   REDIS_ULR="redis_ulr"
   ```
4. Run the app
   ```bash
   npm run start
   ```
   Your app should now be running on http://localhost/3000
5. Once the app is running, open your browser and go to:
  ```arduino
  http://localhost/3000
  ```



