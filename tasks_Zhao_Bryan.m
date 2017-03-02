%% ENG 6 Project 1
% Bryan, Zhao
% Section A11
% 998866097
% Collaborator 1: Jordan Leung
% Collaborator 2: James Borrott

clear; %Clear any previous data
clc; %Clear the command window
load weather.mat; % Load the weather data

% Task 2: 
cityIndex = (city=='D'); % creates a logical array with true values for the letter 'D'
startD = numel(city(cityIndex)); % finds number of true values
fprintf('Task 2: In total, %d cities in the database have names starting with the letter D.\n',startD)

% Task 3: 
uniqueStates = unique(state,'rows'); % finds unique states in variable state
fprintf('Task 3: In total, %d states exist in the database.\n',length(uniqueStates))

% Task 4:
lettersIndex = isletter(city); % creates logical array that determines whether the strings in variable city have spaces
cityNoSpaces = city(lettersIndex); % creates an array of strings with no spaces
i = 0;

% tests for the number of cities that have names longer than 7 characters
% excluding spaces
for n = 1:length(city)
    indexCity = city(n,[1:14]);
    lettersCity = isletter(indexCity);
    noSpacesCity = indexCity(lettersCity);
    cityLength = length(noSpacesCity);
    if cityLength > 7
        i = i + 1;
    end
end
fprintf('Task 4: %d cities in the database have names longer than 7 characters.\n',i)

% Task 5: 
preArray = zeros(1,length(city)); % pre-allocates an array with all zeros

for n = 1:length(city)
    avg = mean(wind(n,:)); % places the mean of wind values by row into the pre-allocated array
    preArray(n) = avg;
end
[mostWind, pos1] = max(preArray); % finds the position of maximum value of all averages
mostWindCity = city(pos1,:);
[leastWind, pos2] = min(preArray);
leastWindCity = city(pos2,:); % finds position of minimum value of all averages
fprintf('Task 5: %s experiences the most wind while %s experiences the least wind.\n',strcat(mostWindCity), strcat(leastWindCity))

% Task 6:
preArray = zeros(1,length(city)); % pre-allocates array with zeros

for n = 1:length(city)
    avg = mean(wind(n,:)); % places mean of wind speeds by row into variable preArray
    preArray(n) = avg;
end

above10 = (preArray>10); % creates logical array with true values that correspond to those greater than 10 mph
citiesAbove10 = preArray(above10);
percentAbove10MPH = (length(citiesAbove10)/length(city))*100; % calculates percentage
fprintf('Task 6: %0.2f percent of cities have monthly wind speed greater than 10 mph.\n',percentAbove10MPH)

% Task 7:
preArray3 = zeros(1,length(city));
wind2 = wind; % to avoid overwriting original variable wind
for n = 1:length(city)
    SumFall = wind2(n,[6:11]);
    SumFall = []; % deletes months that are not in winter and spring
    SprWint = wind2(n,[1:6]);
    avgWind = mean(SprWint); % finds the mean of wind speeds in winter and spring
    preArray3(n) = avgWind; % places resulting averages in the preset array
end
[mostWindWintSpr, indexWintSpr] = max(preArray3); % locates index of highest average
cityWintSpr = city(indexWintSpr,:); % finds city corresponding to index
fprintf('Task 7: %s has the highest wind speed during the winter and spring seasons.\n',strcat(cityWintSpr))

% Task 8:
minval = +inf; % initialize value of variable minval
for k = 1:length(city)
    [leastMonthlySolar index]= min(solar(k,:)); % finds the minimum value and index of each row in variable solar
    if leastMonthlySolar<minval 
        minval = leastMonthlySolar; % finds the lowest value of solar
        solarIndex = index; % month index
        cityIndex = k; % city index
    end
end
switch solarIndex
    case 1
        month = 'January';
    case 2
        month = 'February';
    case 3
        month = 'March';
    case 4
        month = 'April';
    case 5
        month = 'May';
    case 6
        month = 'June';
    case 7
        month = 'July';
    case 8 
        month = 'August';
    case 9 
        month = 'September';
    case 10
        month = 'October';
    case 11
        month = 'November';
    case 12
        month = 'December';
end
fprintf('Task 8: %s has the least monthly solar insolation in %s.\n',strcat(city(cityIndex,:)),month)

% Task 9:
i = 0;
for n = 1:length(city)
    solarIndex = solar(n,:);
    avgSolar = mean(solarIndex); % stores mean values of solar insolation in variable avgSolar
    if avgSolar > 3.9
        i = i+1; % finds number of cities with solar insolation greater than 3.9
    end
end
fprintf('Task 9: %d cities have solar insolation greater than 3.9 kW per square meter.\n',i)

% Task 10:
indexSolar = solar(:,6); % all solar values in the month of June
[val index] = max(indexSolar); % finds max value and its index in June
solarMonth = 6; % month index that corresponds to June
switch solarMonth
    case 1
        month = 'January';
    case 2
        month = 'February';
    case 3
        month = 'March';
    case 4
        month = 'April';
    case 5
        month = 'May';
    case 6
        month = 'June';
    case 7
        month = 'July';
    case 8 
        month = 'August';
    case 9 
        month = 'September';
    case 10
        month = 'October';
    case 11
        month = 'November';
    case 12
        month = 'December';
end
fprintf('Task 10: %s has the largest solar insolation in %s, while ',strcat(city(index,:)),month)
indexSolar2 = solar(:,1); % all solar values in January
[val2 index2] = max(indexSolar2); % finds max value in January and its corresponding city index
solarMonth = 1; % month index (January)
switch solarMonth
    case 1
        month = 'January';
    case 2
        month = 'February';
    case 3
        month = 'March';
    case 4
        month = 'April';
    case 5
        month = 'May';
    case 6
        month = 'June';
    case 7
        month = 'July';
    case 8 
        month = 'August';
    case 9 
        month = 'September';
    case 10
        month = 'October';
    case 11
        month = 'November';
    case 12
        month = 'December';
end
fprintf('%s has the largest in %s.\n',strcat(city(index2,:)),month)

% Task 11:
presetArray = zeros(1,length(city)); % pre-allocate an array with zeros
for n = 1:length(city)
    avgWind = mean(wind(n,:)); % stores averages of all wind values by city
    solarIndex = solar(n,:); 
    avgSolar = mean(solarIndex); % stores averages of all solar values by city
    if (avgWind > 5) & (avgSolar > 3.9) 
        presetArray(n) = avgWind; % finds averages of wind values of cities with solar panels installed (>3.9 kW)
    end
end
[val1 index1] = max(presetArray); % finds top 3 cities that have highest wind values and solar panels installed
presetArray(index1) = 0;
[val2 index2] = max(presetArray);
presetArray(index2) = 0;
[val3 index3] = max(presetArray);
presetArray(index3) = 0;
fprintf('Task 11: In order of likeliness to damage solar panels: %s, %s, %s.\n',strcat(city(index1,:)),strcat(city(index2,:)),strcat(city(index3,:)))

% Task 12:
rainMarch = precip(:,3); % all rain data from March

% finds top 3 cities with most rainwater in March
[valM1 indexM1] = max(rainMarch);
rainMarch(indexM1) = 0;
[valM2 indexM2] = max(rainMarch);
rainMarch(indexM2) = 0;
[valM3 indexM3] = max(rainMarch);
rainMarch(indexM3) = 0;

% finds top 3 cities with most rainwater in December
rainDecember = precip(:,12); % all rain data from December

[valD1 indexD1] = max(rainDecember);
rainDecember(indexD1) = 0;
[valD2 indexD2] = max(rainDecember);
rainDecember(indexD2) = 0;
[valD3 indexD3] = max(rainDecember);
rainDecember(indexD3) = 0;

fprintf('Task 12: %s, %s, and %s experience the most rain in March while while %s, %s, and %s experience the most rain in December.\n',strcat(city(indexM1,:)),strcat(city(indexM2,:)),strcat(city(indexM3,:)),strcat(city(indexD1,:)),strcat(city(indexD2,:)),strcat(city(indexD3,:)))

% Task 13:
precip2=precip;
for n = 1:length(city)
    if wind(n,:) < 3 % if wind speed is below 3 in a city, the corresponding 
% rain data in that city for the month is reduced by 5%
        index = find(wind(n,:)<3);
        precip2(n,index) = precip2(n,index)*.95; 
    end
    if (wind(n,:) >= 3) & (wind(n,:) <=5) % if wind speed is from 3-5 mph in a city in a given month, the corresponding 
% rain data is reduced by 10%
        index2 = find((wind(n,:) >= 3) & (wind(n,:) <=5));
        precip2(n,index2) = precip2(n,index2)*.90;
    end
    if (wind(n,:)) > 5 % if wind speed is above 5 mph in a city for that month, the
% corresponding rain data is reduced by 30%
        index3 = find(wind(n,:)>5);
        precip2(n,index3) = precip2(n,index3)*.70;
    end
end
presetArray = zeros(1,length(city)); % pre-allocates an array filled with zeros
for n = 1:length(city)
    annualRain = mean(precip2(n,:)); % calculates the average of all the adjusted rain data by city
    presetArray(n) = annualRain;
end
[valRain indexRain] = max(presetArray); % finds the index of the city with most collected rainwater (adjusted) in a year
fprintf('Task 13: Accounting for loss due to strong winds. %s collects the most rainwater in a year.\n',strcat(city(indexRain,:)))

% Task 14:
presetArray = zeros(1,length(city)); % pre-allocates an array with zeros
for n = 1:length(city)
    avgRain = mean(precip(n,:)); % calculates averages of rainwater by city
    avgWind = mean(wind(n,:)); % calculates averages of wind speed by city
    solarIndex = solar(n,:);
    avgSolar = mean(solarIndex); % calculates averages of solar data by city

    % finds cities with damaged solar panels (mean rainfall above 2 inches, mean solar insolation
    % above 3.9, and mean wind speed above 5)
    if (avgRain > 2) & (avgSolar > 3.9)
        if (avgSolar > 3.9) & (avgWind > 5)
            presetArray(n) = avgRain; % fills the pre-allocated array with average rainfall values by city
        end
    end
end
% finds the indices of the top 3 cities that are most likely to have
% damaged solar panels
[val1 index1] = max(presetArray); 
presetArray(index1) = 0;
[val2 index2] = max(presetArray);
presetArray(index2) = 0;
[val3 index3] = max(presetArray);
presetArray(index3) = 0;
fprintf('Task 14: In order of likeliness to have solar panels damaged to rain and wind: %s, %s, and %s.\n',strcat(city(index1,:)), strcat(city(index2,:)),strcat(city(index3,:)))

% Task 15:
i = 0; % initialized loop variable
for n = 1:length(city)
    avgWindSpd = mean(wind(n,:)); % finds averages of wind speeds
    solarIndex = solar(n,:);
    avgSolar = mean(solarIndex); % finds averages of solar data
    windToSolar = 2/5; % 2.0 kW of power per wind speed of 5mph
         if avgWindSpd*windToSolar > avgSolar
             i = i+1; % calculates number of cities that have wind power
% greater than solar power
         end
end
fprintf('Task 15: %d cities should install wind turbines.\n',i)

% Task 16:
i=0; % initialized loop variable
for n = 1:length(city)
    avgWindSpd = mean(wind(n,:)); % finds averages of wind speeds
    solarIndex = solar(n,:);
    avgSolar = mean(solarIndex); % finds averages of solar data
    windToSolar = 2/5; % 2.0 kW of power per wind speed of 5mph
         if (avgWindSpd*windToSolar > avgSolar) & (avgSolar > 3.9)
             i = i+1; % number of cities that should install solar panels and wind turbines
         end
end
fprintf('Task 16: %d cities should install both solar panels and wind turbines.\n',i)

% Task 17:
i=0;
avgWindSpd = zeros(1,length(city)); % pre-allocated arrays filled with zeros
avgSolar = zeros(1,length(city));
for n = 1:length(city)
    avgWindSpd(n) = mean(wind(n,:)); % monthly averages of wind data
    solarIndex = solar(n,:);
    avgSolar(n) = mean(solarIndex); % monthly averages of solar data
    annualSolar = avgSolar*12; % solar power (average) in one year
    annualWind = avgWindSpd*12*(2/5); % wind power in one year
    if (annualSolar(n)<42) & (annualWind(n)<42)
        i = i+1; % number of cities that have annual solar and wind power under 42 kW
    end
end
fprintf('Task 17: %d city (cities) are UNSUITABLE to use both wind and solar power. ',i)
avgWindSpd = zeros(1,length(city)); % pre-allocated arrays
avgSolar = zeros(1,length(city));
nameCity = [];

% removes the comma at the end of the display and replaces it with a period
for n = 1:length(city)
    avgWindSpd(n) = mean(wind(n,:));
    solarIndex = solar(n,:);
    avgSolar(n) = mean(solarIndex);
    annualSolar = avgSolar*12;
    annualWind = avgWindSpd*12*(2/5);
    if (annualSolar(n)<42) & (annualWind(n)<42)
        nameCity = [nameCity, ' ',strcat(city(n,:)),','];
    end
end
if nameCity(end) == ','
    nameCity(end) = '.';
end
fprintf('%s',nameCity)

% Task 18:
i=0; % initialized loop variable
avgWindSpd = zeros(1,length(city)); % pre-allocated arrays with zeros
avgSolar = zeros(1,length(city));
for n = 1:length(city)
    avgWindSpd(n) = mean(wind(n,:)); % monthly average of wind data
    solarIndex = solar(n,:);
    avgSolar(n) = mean(solarIndex); % monthly average of solar data
    annualSolar = avgSolar*12; % annual solar data
    annualWind = avgWindSpd*12*(2/5); % annual wind data
    if (annualSolar(n) > 60) | (annualWind(n) > 75)
        i = i+1; % calculates number of cities that are suitable (annual solar > 60 kW and annual wind > 75 kW)
    end
end
fprintf('\nTask 18: %d cities are SUITABLE for at least one type of renewable energy.\n',i)

% Task 19:
i=0; % initialized loop variable
avgWindSpd = zeros(1,length(city)); % pre-allocated arrays
avgSolar = zeros(1,length(city));

% calculates number of cities that have only ONE type of renewable energy
% using the xor function
for n = 1:length(city)
    avgWindSpd(n) = mean(wind(n,:));
    solarIndex = solar(n,:);
    avgSolar(n) = mean(solarIndex);
    annualSolar = avgSolar*12;
    annualWind = avgWindSpd*12*(2/5);
    if xor((annualSolar(n) > 60),(annualWind(n) > 75))
        i = i+1;
    end
end
fprintf('Task 19: %d cities are SUITABLE for ONLY ONE type of renewable energy.\n',i)

% Task 20:
i=0; % initialized loop variable
avgWindSpd = zeros(1,length(city)); %pre-allocated arrays
avgSolar = zeros(1,length(city));
% finds number of cities that are adequate for both wind AND solar power
for n = 1:length(city)
    avgWindSpd(n) = mean(wind(n,:));
    solarIndex = solar(n,:);
    avgSolar(n) = mean(solarIndex);
    annualSolar = avgSolar*12;
    annualWind = avgWindSpd*12*(2/5);
    if ((annualSolar(n) <= 60) & (annualSolar(n) >= 42 )) & ((annualWind(n) <= 75) & (annualWind(n) >= 42))
            i = i+1;
    end
end
fprintf('Task 20: %d cities are ADEQUATE for both solar and wind power.\n',i)

% Task 21:

% finds corresponding y values (rain, wind, and solar data) for each city
% by month
for n = 1:length(city)
    if city(n,:) == 'Washington    '
        rainWashY = precip(n,:);
        windWashY = wind(n,:);
        solarWashY = solar(n,:);
    end
    if city(n,:) == 'Seattle       '
        rainSeaY = precip(n,:);
        windSeaY = wind(n,:);
        solarSeaY = solar(n,:);
    end
    if city(n,:) == 'Phoenix       '
        rainPhnY = precip(n,:);
        windPhnY = wind(n,:);
        solarPhnY = solar(n,:);
    end
end

xAxis = [1:1:12]; % x-values (months)
hold on
% plot data
plot(xAxis,rainWashY,'r:','LineWidth',2.5)
plot(xAxis,windWashY,'r-','LineWidth',2.5)
plot(xAxis,solarWashY,'r-.','LineWidth',2.5)
plot(xAxis,rainSeaY,'b:','LineWidth',2.5)
plot(xAxis,windSeaY,'b-','LineWidth',2.5)
plot(xAxis,solarSeaY,'b-.','LineWidth',2.5)
plot(xAxis,rainPhnY,'m:','LineWidth',2.5)
plot(xAxis,windPhnY,'m-','LineWidth',2.5)
plot(xAxis,solarPhnY,'m-.','LineWidth',2.5)
legend ('Precipitation for Washington','Wind for Washington', 'Solar for Washington', 'Precipitation for Seattle', 'Wind for Seattle', 'Solar for Seattle', 'Precipitation for Phoenix', 'Wind for Phoenix', 'Solar for Phoenix')
xlabel('Months from January to December')
ylabel('Rates of power (rainfall, MPH, kW)')




