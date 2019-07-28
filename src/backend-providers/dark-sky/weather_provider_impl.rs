use forecast::{ApiClient, ApiResponse, DataPoint, ExtendBy, ForecastRequestBuilder, Icon, Lang, Units};

use lib_backend_common::{WeatherForecast, WeatherIcon, WeatherProvider, WeatherStatus};

use crate::Provider;

impl Provider {
    fn weather(&self) -> ApiResponse {
        let request = ForecastRequestBuilder::new(&self.config.api_key, self.config.latitude, self.config.longitude)
            .extend(ExtendBy::Hourly)
            .lang(Lang::English)
            .units(Units::Auto)
            .build();

        ApiClient::new(&self.client)
            .get_forecast(request)
            .unwrap()
            .json()
            .unwrap()
    }
}

impl WeatherProvider for Provider {
    fn name(&self) -> &'static str {
        "dark-sky"
    }

    fn current(&mut self) -> WeatherStatus {
        self.weather()
            .currently
            .map(build_weather_status)
            .unwrap_or_else(WeatherStatus::default)
    }

    fn forecast(&mut self) -> Option<WeatherForecast> {
        unimplemented!()
    }
}

fn build_weather_status(dp: DataPoint) -> WeatherStatus {
    WeatherStatus {
        temperature: dp.temperature.map(|v| v as f32),
        pressure: dp.pressure.map(|v| v as i32),
        humidity: dp.humidity.map(|v| (v * 100.0) as i32),
        wind_speed: dp.wind_speed.map(|v| v as f32),

        icon: dp.icon.as_ref().map(|icon| {
            match icon {
                Icon::ClearDay | Icon::ClearNight => WeatherIcon::Sun,
                Icon::Rain => WeatherIcon::Rain,
                Icon::Snow | Icon::Sleet => WeatherIcon::Snow,
                Icon::Wind => WeatherIcon::SunWithCloud,
                Icon::Fog => WeatherIcon::Fog,
                Icon::Cloudy | Icon::PartlyCloudyDay | Icon::PartlyCloudyNight => WeatherIcon::SunWithCloud,
                Icon::Hail | Icon::Thunderstorm | Icon::Tornado => WeatherIcon::Thunderstorm,
            }
        }),
    }
}