function convertLength(value, fromUnit, toUnit) {
    // Convert fromUnit to meters
    let inMeters;
    switch (fromUnit) {
        case 'millimeter':
            inMeters = value / 1000;
            break;
        case 'centimeter':
            inMeters = value / 100;
            break;
        case 'meter':
            inMeters = value;
            break;
        case 'kilometer':
            inMeters = value * 1000;
            break;
        case 'inch':
            inMeters = value * 0.0254;
            break;
        case 'foot':
            inMeters = value * 0.3048;
            break;
        case 'yard':
            inMeters = value * 0.9144;
            break;
        case 'mile':
            inMeters = value * 1609.34;
            break;
        default:
            throw new Error('Unsupported length unit');
    }

    // Convert meters to toUnit
    switch (toUnit) {
        case 'millimeter':
            return inMeters * 1000;
        case 'centimeter':
            return inMeters * 100;
        case 'meter':
            return inMeters;
        case 'kilometer':
            return inMeters / 1000;
        case 'inch':
            return inMeters / 0.0254;
        case 'foot':
            return inMeters / 0.3048;
        case 'yard':
            return inMeters / 0.9144;
        case 'mile':
            return inMeters / 1609.34;
        default:
            throw new Error('Unsupported length unit');
    }
}

function convertWeight(value, fromUnit, toUnit) {
    // Convert fromUnit to kilograms
    let inKilograms;
    switch (fromUnit) {
        case 'milligram':
            inKilograms = value / 1_000_000;
            break;
        case 'gram':
            inKilograms = value / 1000;
            break;
        case 'kilogram':
            inKilograms = value;
            break;
        case 'ounce':
            inKilograms = value * 0.0283495;
            break;
        case 'pound':
            inKilograms = value * 0.453592;
            break;
        default:
            throw new Error('Unsupported weight unit');
    }

    // Convert kilograms to toUnit
    switch (toUnit) {
        case 'milligram':
            return inKilograms * 1_000_000;
        case 'gram':
            return inKilograms * 1000;
        case 'kilogram':
            return inKilograms;
        case 'ounce':
            return inKilograms / 0.0283495;
        case 'pound':
            return inKilograms / 0.453592;
        default:
            throw new Error('Unsupported weight unit');
    }
}

function convertTemperature(value, fromUnit, toUnit) {
    let inCelsius;

    // Convert fromUnit to Celsius
    switch (fromUnit) {
        case 'celsius':
            inCelsius = value;
            break;
        case 'fahrenheit':
            inCelsius = (value - 32) * (5 / 9);
            break;
        case 'kelvin':
            inCelsius = value - 273.15;
            break;
        default:
            throw new Error('Unsupported temperature unit');
    }

    // Convert Celsius to toUnit
    switch (toUnit) {
        case 'celsius':
            return inCelsius;
        case 'fahrenheit':
            return (inCelsius * (9 / 5)) + 32;
        case 'kelvin':
            return inCelsius + 273.15;
        default:
            throw new Error('Unsupported temperature unit');
    }
}

