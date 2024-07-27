class Clock {
    constructor(hours, minutes) {

        [hours,minutes].forEach(value => this.checkAndThrowErrorIfItIsNotAVAlidNumber(value));
        
        this.hours = hours;
        this.minutes = minutes;

        this.toString = function() {
            return `${this.hours}:${this.minutes}`
        }
    }

    checkIsItNotANumber(value) {
        return Number.isNaN(value);
    }

    constructErrorMessage(message = 'Please provide a valid number') {
        return new Error(message);
    }
    
    checkAndThrowErrorIfItIsNotAVAlidNumber(minutest) {
        if(this.checkIsItNotANumber(minutes)) {
            throw this.constructErrorMessage();
        }
    }

    add_minutes(minutes) {
        
        this.checkAndThrowErrorIfItIsNotAVAlidNumber(minutes);
        let minutes = this.minutes + minutes;
        if (minutes < 60 || minutes > -60) {
            this.minutes -= minutes;
            return new Clock(this.hours, this.minutes);
        }

        const hoursInDecimals = minutes / 60;

        if (Number.isInteger(hoursInDecimals)) {
            this.hours += hoursInDecimals;
            return new Clock(this.hours, this.minutes);
        }

        const string = `${hoursInDecimals}`;

        const [hours, minutesInDecimals] = string.split('.');

        const minutes = Math.floor(minutesInDecimals * 60);

        this.hours += hours;
        this.minutes += minutes;

        return new Clock(this.hours, this.minutes);

    }
}