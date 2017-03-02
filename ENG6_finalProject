function varargout = gui(varargin)
% GUI MATLAB code for gui.fig
%      GUI, by itself, creates a new GUI or raises the existing
%      singleton*.
%
%      H = GUI returns the handle to a new GUI or the handle to
%      the existing singleton*.
%
%      GUI('CALLBACK',hObject,eventData,handles,...) calls the local
%      function named CALLBACK in GUI.M with the given input arguments.
%
%      GUI('Property','Value',...) creates a new GUI or raises the
%      existing singleton*.  Starting from the left, property value pairs are
%      applied to the GUI before gui_OpeningFcn gets called.  An
%      unrecognized property name or invalid value makes property application
%      stop.  All inputs are passed to gui_OpeningFcn via varargin.
%
%      *See GUI Options on GUIDE's Tools menu.  Choose "GUI allows only one
%      instance to run (singleton)".
%
% See also: GUIDE, GUIDATA, GUIHANDLES
 
% Edit the above text to modify the response to help gui
 
% Last Modified by GUIDE v2.5 17-Mar-2014 12:56:20
 
% Begin initialization code - DO NOT EDIT
gui_Singleton = 1;
gui_State = struct('gui_Name',       mfilename, ...
                   'gui_Singleton',  gui_Singleton, ...
                   'gui_OpeningFcn', @gui_OpeningFcn, ...
                   'gui_OutputFcn',  @gui_OutputFcn, ...
                   'gui_LayoutFcn',  [] , ...
                   'gui_Callback',   []);
if nargin && ischar(varargin{1})
    gui_State.gui_Callback = str2func(varargin{1});
end
 
if nargout
    [varargout{1:nargout}] = gui_mainfcn(gui_State, varargin{:});
else
    gui_mainfcn(gui_State, varargin{:});
end
% End initialization code - DO NOT EDIT
 
 
% --- Executes just before gui is made visible.
function gui_OpeningFcn(hObject, eventdata, handles, varargin)
% This function has no output args, see OutputFcn.
% hObject    handle to figure
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
%% PANEL AND TURBINE LIST
uiwait(msgbox('Please wait for all of the color boxes to load. Click ''OK'' to load the GUI.'));
handles.b = getappdata(0,'ReturnText');
b= handles.b;
switch b
    case 1
        handles.map=imread('Chicago.bmp'); %% Using sample map
        size_of_box=10; %% 10x10
        % Create boxes on the map.
        handles.map(1:size_of_box:end,:,:)=0;
        handles.map(:,1:size_of_box:end,:)=0;
        handles.originalMap = handles.map;
        imshow(handles.map);
        set(handles.cityName,'String','Chicago');
    case 2
        handles.map=imread('SanFrancisco.bmp'); %% Using sample map
        size_of_box=10; %% 10x10
        % Create boxes on the map.
        handles.map(1:size_of_box:end,:,:)=0;
        handles.map(:,1:size_of_box:end,:)=0;
        handles.originalMap = handles.map;
        imshow(handles.map);
        set(handles.cityName,'String','San Francisco');
    case 3
        handles.map=imread('Denver.bmp'); %% Using sample map
        size_of_box=10; %% 10x10
        % Create boxes on the map.
        handles.map(1:size_of_box:end,:,:)=0;
        handles.map(:,1:size_of_box:end,:)=0;
        handles.originalMap = handles.map;
        imshow(handles.map);
        set(handles.cityName,'String','Denver');
end
[handles.sizeC,handles.sizeR,color] = size(handles.map);
handles.output = hObject;
switch b
    case 1
        handles.city = {'Chicago'};
        handles.averageSolar = {3.7283};
        handles.averageWind = {8.75};
    case 2
        handles.city = {'San Francisco'};
        handles.averageSolar = {4.8933};
        handles.averageWind = {10};
    case 3
        handles.city = {'Denver'};
        handles.averageSolar = {4.5508};
        handles.averageWind = {7.5};
end
handles.averageSolar{1,1} = handles.averageSolar{1,1}./10.7639;

handles.panelBrand = {'Sunforce 50048';'Sunforce 39810';'Instapark SPCC-5W';'Instapark SP-100W';'Instapark SPCC-30W';'Instapark SP-10W';'Ramsond 100SP';'Epcom WK50-12';'Sun Power E18';'Sun Power T5'};
handles.dimension = {42.5,16;21,48;11,8;45,26;21.5,17.2;14,11;45,21.8;32,22;41.18,81.36;43.06,75.13};
handles.panelArea = cell(1,10);
for i=1:1:10
    handles.panelArea{1,i} = (handles.dimension{i,1}*handles.dimension{i,2})/144;
end
handles.panelPrice = {279.95;499.95;34.95;319.99;114.70;39.95;245.99;99.99;249.5;199.99};
handles.turbineBrand = {'Windmax HY 1000-5';'Windmax HY 400';'GudCraft WG400';'GudCraft WG700';'All Power America APWT400A';'Sunforce 45444';'Sunforce 44444';'WindyNation WCK-750'};
handles.diameter = {15;13;13;13;10;10;10;15};
handles.turbineArea = cell(1,8);
for i=1:1:8
    handles.turbineArea{1,i} = ((handles.diameter{i}/2)*(handles.diameter{i}/2))*pi;
end
handles.turbinePrice = {999.99;686.40;399.00;449.00;476.93;749.99;474.34;999.38};



%% Display boxes
axes(handles.imPanel1);

blankSquare1(:,:,1) = 0;
blankSquare1(:,:,2) = 0;
blankSquare1(:,:,3) = 255/255;
imshow(blankSquare1)

axes(handles.imPanel2);

blankSquare2(:,:,1) =100/255;
blankSquare2(:,:,2) = 149/255;
blankSquare2(:,:,3) = 237/255;
imshow(blankSquare2)

axes(handles.imPanel3);
blankSquare3 = zeros(10,10);
blankSquare3(:,:,1) = 0;
blankSquare3(:,:,2) = 0;
blankSquare3(:,:,3) = 128/255;
imshow(blankSquare3)

axes(handles.imPanel4);
blankSquare4 = zeros(10,10);
blankSquare4(:,:,1) =30/255;
blankSquare4(:,:,2) = 144/255;
blankSquare4(:,:,3) = 255/255;
imshow(blankSquare4)

axes(handles.imPanel5);
blankSquare5 = zeros(10,10);
blankSquare5(:,:,1) =0;
blankSquare5(:,:,2) = 206/255;
blankSquare5(:,:,3) = 209/255;
imshow(blankSquare5)

axes(handles.imPanel6);
blankSquare6 = zeros(10,10);
blankSquare6(:,:,1) =150/255;
blankSquare6(:,:,2) = 205/255;
blankSquare6(:,:,3) = 205/255;
imshow(blankSquare6)

axes(handles.imPanel7);
blankSquare7 = zeros(10,10);
blankSquare7(:,:,1) =0;
blankSquare7(:,:,2) = 199/255;
blankSquare7(:,:,3) = 140/255;
imshow(blankSquare7)

axes(handles.imPanel8);
blankSquare8 = zeros(10,10);
blankSquare8(:,:,1) =69/255;
blankSquare8(:,:,2) = 139/255;
blankSquare8(:,:,3) = 116/255;
imshow(blankSquare8)

axes(handles.imPanel9);
blankSquare9 = zeros(10,10);
blankSquare9(:,:,1) =148/255;
blankSquare9(:,:,2) = 188/255;
blankSquare9(:,:,3) = 143/255;
imshow(blankSquare9)

axes(handles.imPanel10);
blankSquare10 = zeros(10,10);
blankSquare10(:,:,1) =118/255;
blankSquare10(:,:,2) = 238/255;
blankSquare10(:,:,3) = 0;
imshow(blankSquare10)

axes(handles.imTurbine1);
blankSquare11 = zeros(10,10);
blankSquare11(:,:,1) =255/255;
blankSquare11(:,:,2) = 255/255;
blankSquare11(:,:,3) = 0;
imshow(blankSquare11)

axes(handles.imTurbine2);
blankSquare12 = zeros(10,10);
blankSquare12(:,:,1) =205/255;
blankSquare12(:,:,2) = 173/255;
blankSquare12(:,:,3) = 0;
imshow(blankSquare12)

axes(handles.imTurbine3);
blankSquare13 = zeros(10,10);
blankSquare13(:,:,1) =139/255;
blankSquare13(:,:,2) = 117/255;
blankSquare13(:,:,3) = 0;
imshow(blankSquare13)

axes(handles.imTurbine4);
blankSquare14 = zeros(10,10);
blankSquare14(:,:,1) =139/255;
blankSquare14(:,:,2) = 69/255;
blankSquare14(:,:,3) = 19/255;
imshow(blankSquare14)

axes(handles.imTurbine5);
blankSquare15 = zeros(10,10);
blankSquare15(:,:,1) = 255/255;
blankSquare15(:,:,2) = 97/255;
blankSquare15(:,:,3) = 3/255;
imshow(blankSquare15)

axes(handles.imTurbine6);
blankSquare16 = zeros(10,10);
blankSquare16(:,:,1) = 255/255;
blankSquare16(:,:,2) = 130/255;
blankSquare16(:,:,3) = 71/255;
imshow(blankSquare16)

axes(handles.imTurbine7);
blankSquare17 = zeros(10,10);
blankSquare17(:,:,1) = 255/255;
blankSquare17(:,:,2) = 69/255;
blankSquare17(:,:,3) = 0;
imshow(blankSquare17)

axes(handles.imTurbine8);
blankSquare18 = zeros(10,10);
blankSquare18(:,:,1) =205/255;
blankSquare18(:,:,2) = 85/255;
blankSquare18(:,:,3) = 85/255;
imshow(blankSquare18)


%%


% Find the point where the user clicks on the map.

 
% varargin   command line arguments to gui (see VARARGIN)
 
% Choose default command line output for gui
handles.output = hObject;
% Update handles structure
guidata(hObject, handles);
% UIWAIT makes gui wait for user response (see UIRESUME)
% uiwait(handles.figure1);
 
 
% --- Outputs from this function are returned to the command line.
function varargout = gui_OutputFcn(hObject, eventdata, handles) 
% varargout  cell array for returning output args (see VARARGOUT);
% hObject    handle to figure
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
 
% Get default command line output from handles structure
varargout{1} = handles.output;
 
 
% --- Executes on selection change in panelPopUp.
function panelPopUp_Callback(hObject, eventdata, handles)
% hObject    handle to panelPopUp (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
 
% Hints: contents = cellstr(get(hObject,'String')) returns panelPopUp contents as cell array
%        contents{get(hObject,'Value')} returns selected item from panelPopUp
%% UPDATE PANEL NUMBER

contents = cellstr(get(hObject,'String'));
a = contents{get(hObject,'Value')};
b = get(hObject,'Value');
[xCoordinate, yCoordinate] = ginput(1);


%%


% Updates the number of remaining boxes
if ((xCoordinate <790) & (yCoordinate <460))&((xCoordinate >=0)&(yCoordinate >=0))
    
%%Color

size_of_box=10;
xCoordinate = fix(xCoordinate);
yCoordinate = fix(yCoordinate);
mapgrid=handles.map;
mapgrid(:)=255;
mapgrid(1:size_of_box:end,:)=0;
mapgrid(:,1:size_of_box:end)=0;

 switch b
    case 2
        colorBox1 = 0;
        colorBox2 = 0;
        colorBox3 = 255;
    case 3
        colorBox1 = 100;
        colorBox2 = 149;
        colorBox3 = 237;
    case 4
        colorBox1 = 0;
        colorBox2 = 0;
        colorBox3 = 128;
    case 5
        colorBox1 = 30;
        colorBox2 = 144;
        colorBox3 = 255;
    case 6
        colorBox1 = 0;
        colorBox2 = 206;
        colorBox3 = 209;
    case 7
        colorBox1 = 150;
        colorBox2 = 205;
        colorBox3 = 205;
    case 8
        colorBox1 = 0;
        colorBox2 = 199;
        colorBox3 = 140;
    case 9
        colorBox1 = 69;
        colorBox2 = 139;
        colorBox3 = 116;
     case 10
         colorBox1 = 148;
         colorBox2 = 188;
         colorBox3 = 143;
     case 11
         colorBox1 = 118;
         colorBox2 = 238;
         colorBox3 = 0;
 end

 

% Find where the top horizontal axis is.
tmp=0;
counter = 0;
i=fix(yCoordinate-size_of_box);
if i<=0
    i=0;
elseif mapgrid(i,xCoordinate)==0
    uiwait(msgbox('Please click within the map','Error','error'));
    tmp=tmp+1;
else
while mapgrid(i,xCoordinate)~=0
    i=i+1;
end
end
% Find where the left vertical axis of the box
ii=fix(xCoordinate-size_of_box);
if ii<=0
    ii=0;
else
while mapgrid(yCoordinate,ii)~=0 
    ii=ii+1;
end
end
% Change the color of the box and display the new map.
delete =0;
if (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) == 255)
        number = get(handles.noPanel1,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,1});
        number = number - number1;
        set(handles.noPanel1,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,1});
        price = handles.panelPrice{1}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 100)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 149)& (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==237)
        number = get(handles.noPanel2,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,2});
        number = number - number1;
        set(handles.noPanel2,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,2});
        price = handles.panelPrice{2}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 0)& (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==128)
        number = get(handles.noPanel3,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,3});
        number = number - number1;
        set(handles.noPanel3,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,3});
        price = handles.panelPrice{3}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 30)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 144)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==255
        number = get(handles.noPanel4,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,4});
        number = number - number1;
        set(handles.noPanel4,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,4});
        price = handles.panelPrice{4}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 206)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==209
        number = get(handles.noPanel5,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,5});
        number = number - number1;
        set(handles.noPanel5,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,5});
        price = handles.panelPrice{5}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 150)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 205)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==205
        number = get(handles.noPanel6,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,6});
        number = number - number1;
        set(handles.noPanel6,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,6});
        price = handles.panelPrice{6}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 199)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==140
        number = get(handles.noPanel7,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,7});
        number = number - number1;
        set(handles.noPanel7,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,7});
        price = handles.panelPrice{7}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 69)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 139)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==116
        number = get(handles.noPanel8,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,8});
        number = number - number1;
        set(handles.noPanel8,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,8});
        price = handles.panelPrice{8}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 148)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 188)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==143
        number = get(handles.noPanel9,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,9});
        number = number - number1;
        set(handles.noPanel9,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,9});
        price = handles.panelPrice{9}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 118)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 238)&handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==0
        number = get(handles.noPanel10,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,10});
        number = number - number1;
        set(handles.noPanel10,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,10});
        price = handles.panelPrice{10}*number1;
        delete = 1;
end
if delete == 1
totalNumber = str2num(get(handles.totalSolarPanels,'String'));
totalNumber = totalNumber - number1;
set(handles.totalSolarPanels,'String',num2str(totalNumber));
solarEnergy = str2num(get(handles.noSolarEstimate,'String'));
solarEnergy = round(solarEnergy - solarEnergyDeduct);
set(handles.noSolarEstimate,'String',num2str(solarEnergy));
totalPrice = str2num(get(handles.totalPrice,'String'));
totalPrice = round(totalPrice - price);
set(handles.totalPrice,'String',num2str(totalPrice));
oldRemBoxesString = get(handles.noRemainingBoxes,'String');
oldRemBoxes = str2num(oldRemBoxesString);
updateRemBoxes = oldRemBoxes + 1;
set(handles.noRemainingBoxes,'String',num2str(updateRemBoxes))
end
delete = 0;
if handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==0
    number = get(handles.noTurbine1,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,1});
    number = number - number1;
    set(handles.noTurbine1,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{1}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==205 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==173 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==0
    number = get(handles.noTurbine2,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,2});
    number = number - number1;
    set(handles.noTurbine2,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{2}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==139 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==117 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==0
    number = get(handles.noTurbine3,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,3});
    number = number - number1;
    set(handles.noTurbine3,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{3}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==139 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==69 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==19
    number = get(handles.noTurbine4,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,4});
    number = number - number1;
    set(handles.noTurbine4,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{4}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==97 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==3
    number = get(handles.noTurbine5,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,5});
    number = number - number1;
    set(handles.noTurbine5,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{5}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==130 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==71
    number = get(handles.noTurbine6,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,6});
    number = number - number1;
    set(handles.noTurbine6,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{6}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==69 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==0
    number = get(handles.noTurbine7,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,7});
    number = number - number1;
    set(handles.noTurbine7,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{7}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==205 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==85 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==85
    number = get(handles.noTurbine8,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,8});
    number = number - number1;
    set(handles.noTurbine8,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{8}*number1;
    delete = 1;
end
if delete ==1
    totalNumber = str2num(get(handles.totalWindTurbines,'String'));
    totalNumber = totalNumber - number1;
    set(handles.totalWindTurbines,'String',num2str(totalNumber));
    windEnergy = str2num(get(handles.noWindEstimate,'String'));
    windEnergy = round(windEnergy - windEnergyDeduct);
    set(handles.noWindEstimate,'String',num2str(windEnergy));
    totalPrice = str2num(get(handles.totalPrice,'String'));
    totalPrice = round(totalPrice - price);
    set(handles.totalPrice,'String',num2str(totalPrice));
    oldRemBoxesString = get(handles.noRemainingBoxes,'String');
    oldRemBoxes = str2num(oldRemBoxesString);
    updateRemBoxes = oldRemBoxes + 1;
    set(handles.noRemainingBoxes,'String',num2str(updateRemBoxes))
end
remainingBoxes = get(handles.noRemainingBoxes,'String');
remainingBoxes = str2num(remainingBoxes);
if remainingBoxes == 0
    msgbox('You cannot select more than 20 boxes','Error','error');
else
if tmp==0
    handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)=colorBox1; %% Set the color
    handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)=colorBox2;
    handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)=colorBox3;
    counter = 1;
end   



if counter == 1
    %%
oldRemBoxesString = get(handles.noRemainingBoxes,'String');
oldRemBoxes = str2num(oldRemBoxesString);
updateRemBoxes = oldRemBoxes - 1;
set(handles.noRemainingBoxes,'String',num2str(updateRemBoxes));

switch b
    case 2
        number = get(handles.noPanel1,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,1});
        number = number + number1;
        set(handles.noPanel1,'String',num2str(number));
        solarEnergyAdd = handles.averageSolar{1,1}*number1*handles.panelArea{1,1};
        price = handles.panelPrice{1}*number1;
    case 3
        number = get(handles.noPanel2,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,2});
        number = number + number1;
        set(handles.noPanel2,'String',num2str(number));
        solarEnergyAdd = handles.averageSolar{1,1}*number1*handles.panelArea{1,2};
        price = handles.panelPrice{2}*number1;
    case 4
        number = get(handles.noPanel3,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,3});
        number = number + number1;
        set(handles.noPanel3,'String',num2str(number));
        solarEnergyAdd = handles.averageSolar{1,1}*number1*handles.panelArea{1,3};
        price = handles.panelPrice{3}*number1;
    case 5
        number = get(handles.noPanel4,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,4});
        number = number + number1;
        set(handles.noPanel4,'String',num2str(number));
        solarEnergyAdd = handles.averageSolar{1,1}*number1*handles.panelArea{1,4};
        price = handles.panelPrice{4}*number1;
    case 6
        number = get(handles.noPanel5,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,5});
        number = number + number1;
        set(handles.noPanel5,'String',num2str(number));
        solarEnergyAdd = handles.averageSolar{1,1}*number1*handles.panelArea{1,5};
        price = handles.panelPrice{5}*number1;
    case 7
        number = get(handles.noPanel6,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,6});
        number = number + number1;
        set(handles.noPanel6,'String',num2str(number));
        solarEnergyAdd = handles.averageSolar{1,1}*number1*handles.panelArea{1,6};
        price = handles.panelPrice{6}*number1;
    case 8
        number = get(handles.noPanel7,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,7});
        number = number + number1;
        set(handles.noPanel7,'String',num2str(number));
        solarEnergyAdd = handles.averageSolar{1,1}*number1*handles.panelArea{1,7};
        price = handles.panelPrice{7}*number1;
    case 9
        number = get(handles.noPanel8,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,8});
        number = number + number1;
        set(handles.noPanel8,'String',num2str(number));
        solarEnergyAdd = handles.averageSolar{1,1}*number1*handles.panelArea{1,8};
        price = handles.panelPrice{8}*number1;
    case 10
        number = get(handles.noPanel9,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,9});
        number = number + number1;
        set(handles.noPanel9,'String',num2str(number));
        solarEnergyAdd = handles.averageSolar{1,1}*number1*handles.panelArea{1,9};
        price = handles.panelPrice{9}*number1;
    case 11
        number = get(handles.noPanel10,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,10});
        number = number + number1;
        set(handles.noPanel10,'String',num2str(number));
        solarEnergyAdd = handles.averageSolar{1,1}*number1*handles.panelArea{1,10};
        price = handles.panelPrice{10}*number1;
end
totalNumber = str2num(get(handles.totalSolarPanels,'String'));
totalNumber = totalNumber + number1;
set(handles.totalSolarPanels,'String',num2str(totalNumber));
solarEnergy = str2num(get(handles.noSolarEstimate,'String'));
solarEnergy = round(solarEnergy + solarEnergyAdd);
set(handles.noSolarEstimate,'String',num2str(solarEnergy));
totalPrice = str2num(get(handles.totalPrice,'String'));
totalPrice = round(totalPrice + price);
set(handles.totalPrice,'String',num2str(totalPrice));
end
end
else
     msgbox('Error. Please try again. Hint: Did you click inside of the map?','Error','error')
end

imshow(handles.map);

guidata(hObject,handles);
 
% --- Executes during object creation, after setting all properties.
function panelPopUp_CreateFcn(hObject, eventdata, handles)
% hObject    handle to panelPopUp (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called
 
% Hint: popupmenu controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end
 
 
% --- Executes on selection change in turbinePopUp.
function turbinePopUp_Callback(hObject, eventdata, handles)
% hObject    handle to turbinePopUp (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
 
% Hints: contents = cellstr(get(hObject,'String')) returns turbinePopUp contents as cell array
%        contents{get(hObject,'Value')} returns selected item from turbinePopUp



%%

contents = cellstr(get(hObject,'String'));
a = contents{get(hObject,'Value')};
b = get(hObject,'Value');
[xCoordinate, yCoordinate] = ginput(1);


%%
 
 
% Updates the number of remaining boxes


if ((xCoordinate <790) & (yCoordinate <460))&((xCoordinate >=0)&(yCoordinate >=0))
    
    


%% 

size_of_box=10;
xCoordinate = round(xCoordinate);
yCoordinate = round(yCoordinate);
mapgrid=handles.map;
mapgrid(:)=255;
mapgrid(1:size_of_box:end,:)=0;
mapgrid(:,1:size_of_box:end)=0;


%% color to change

switch b
    case 2
        colorBox1 = 255;
        colorBox2 = 255;
        colorBox3 = 0;
    case 3
        colorBox1 = 205;
        colorBox2 = 173;
        colorBox3 = 0;
    case 4
        colorBox1 = 139;
        colorBox2 = 117;
        colorBox3 = 0;
    case 5
        colorBox1 = 139;
        colorBox2 = 69;
        colorBox3 = 19;
    case 6
        colorBox1 = 255;
        colorBox2 = 97;
        colorBox3 = 3;
    case 7
        colorBox1 = 255;
        colorBox2 = 130;
        colorBox3 = 71;
    case 8
        colorBox1 = 255;
        colorBox2 = 69;
        colorBox3 = 0;
    case 9
        colorBox1 = 205;
        colorBox2 = 85;
        colorBox3 = 85;
end


 
% Find where the top horizontal axis is.
tmp=0;
counter = 0;
i=round(yCoordinate-size_of_box);
if i<=0
    i=0;
elseif mapgrid(i,xCoordinate)==0
    uiwait(msgbox('Please click within the box','Error','error'));
    tmp=tmp+1;
else
while mapgrid(i,xCoordinate)~=0
    i=i+1;
end
end
% Find where the left vertical axis of the box is
ii=round(xCoordinate-size_of_box);
if ii<=0
    ii=0;
else
while mapgrid(yCoordinate,ii)~=0 
    ii=ii+1;
end
end
delete =0;
if (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) == 255)
        number = get(handles.noPanel1,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,1});
        number = number - number1;
        set(handles.noPanel1,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,1});
        price = handles.panelPrice{1}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 100)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 149)& (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==237)
        number = get(handles.noPanel2,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,2});
        number = number - number1;
        set(handles.noPanel2,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,2});
        price = handles.panelPrice{2}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 0)& (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==128)
        number = get(handles.noPanel3,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,3});
        number = number - number1;
        set(handles.noPanel3,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,3});
        price = handles.panelPrice{3}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 30)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 144)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==255
        number = get(handles.noPanel4,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,4});
        number = number - number1;
        set(handles.noPanel4,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,4});
        price = handles.panelPrice{4}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 206)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==209
        number = get(handles.noPanel5,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,5});
        number = number - number1;
        set(handles.noPanel5,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,5});
        price = handles.panelPrice{5}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 150)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 205)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==205
        number = get(handles.noPanel6,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,6});
        number = number - number1;
        set(handles.noPanel6,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,6});
        price = handles.panelPrice{6}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 199)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==140
        number = get(handles.noPanel7,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,7});
        number = number - number1;
        set(handles.noPanel7,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,7});
        price = handles.panelPrice{7}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 69)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 139)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==116
        number = get(handles.noPanel8,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,8});
        number = number - number1;
        set(handles.noPanel8,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,8});
        price = handles.panelPrice{8}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 148)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 188)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==143
        number = get(handles.noPanel9,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,9});
        number = number - number1;
        set(handles.noPanel9,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,9});
        price = handles.panelPrice{9}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 118)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 238)&handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==0
        number = get(handles.noPanel10,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,10});
        number = number - number1;
        set(handles.noPanel10,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,10});
        price = handles.panelPrice{10}*number1;
        delete = 1;
end
if delete == 1
totalNumber = str2num(get(handles.totalSolarPanels,'String'));
totalNumber = totalNumber - number1;
set(handles.totalSolarPanels,'String',num2str(totalNumber));
solarEnergy = str2num(get(handles.noSolarEstimate,'String'));
solarEnergy = round(solarEnergy - solarEnergyDeduct);
set(handles.noSolarEstimate,'String',num2str(solarEnergy));
totalPrice = str2num(get(handles.totalPrice,'String'));
totalPrice = round(totalPrice - price);
set(handles.totalPrice,'String',num2str(totalPrice));
oldRemBoxesString = get(handles.noRemainingBoxes,'String');
oldRemBoxes = str2num(oldRemBoxesString);
updateRemBoxes = oldRemBoxes + 1;
set(handles.noRemainingBoxes,'String',num2str(updateRemBoxes))
end
delete = 0;
if handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==0
    number = get(handles.noTurbine1,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,1});
    number = number - number1;
    set(handles.noTurbine1,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{1}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==205 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==173 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==0
    number = get(handles.noTurbine2,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,2});
    number = number - number1;
    set(handles.noTurbine2,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{2}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==139 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==117 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==0
    number = get(handles.noTurbine3,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,3});
    number = number - number1;
    set(handles.noTurbine3,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{3}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==139 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==69 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==19
    number = get(handles.noTurbine4,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,4});
    number = number - number1;
    set(handles.noTurbine4,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{4}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==97 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==3
    number = get(handles.noTurbine5,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,5});
    number = number - number1;
    set(handles.noTurbine5,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{5}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==130 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==71
    number = get(handles.noTurbine6,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,6});
    number = number - number1;
    set(handles.noTurbine6,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{6}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==69 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==0
    number = get(handles.noTurbine7,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,7});
    number = number - number1;
    set(handles.noTurbine7,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{7}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==205 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==85 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==85
    number = get(handles.noTurbine8,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,8});
    number = number - number1;
    set(handles.noTurbine8,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{8}*number1;
    delete = 1;
end
if delete ==1
    totalNumber = str2num(get(handles.totalWindTurbines,'String'));
    totalNumber = totalNumber - number1;
    set(handles.totalWindTurbines,'String',num2str(totalNumber));
    windEnergy = str2num(get(handles.noWindEstimate,'String'));
    windEnergy = round(windEnergy - windEnergyDeduct);
    set(handles.noWindEstimate,'String',num2str(windEnergy));
    totalPrice = str2num(get(handles.totalPrice,'String'));
    totalPrice = round(totalPrice - price);
    set(handles.totalPrice,'String',num2str(totalPrice));
    oldRemBoxesString = get(handles.noRemainingBoxes,'String');
    oldRemBoxes = str2num(oldRemBoxesString);
    updateRemBoxes = oldRemBoxes + 1;
    set(handles.noRemainingBoxes,'String',num2str(updateRemBoxes))
end
remainingBoxes = get(handles.noRemainingBoxes,'String');
remainingBoxes = str2num(remainingBoxes);
if remainingBoxes == 0
    msgbox('You cannot select more than 20 boxes','Error','error');
else
% Change the color of the box and display the new map.
if tmp==0
    handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)=colorBox1;%% Set the color
    handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)=colorBox2;
    handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)=colorBox3;
    counter = 1;
end

%% 
if counter ==1
oldRemBoxesString = get(handles.noRemainingBoxes,'String');
oldRemBoxes = str2num(oldRemBoxesString);
updateRemBoxes = oldRemBoxes - 1;
set(handles.noRemainingBoxes,'String',num2str(updateRemBoxes))
switch b
    case 2
        number = get(handles.noTurbine1,'String');
        number = str2num(number);
        number1 = fix(225/handles.turbineArea{1,1});
        number = number + number1;
        set(handles.noTurbine1,'String',num2str(number));
        windEnergyAdd = handles.averageWind{1,1}*number1*500;
        price = handles.turbinePrice{1}*number1;
    case 3
        number = get(handles.noTurbine2,'String');
        number = str2num(number);
        number1 = fix(225/handles.turbineArea{1,2});
        number = number + number1;
        set(handles.noTurbine2,'String',num2str(number));
        windEnergyAdd = handles.averageWind{1,1}*number1*500;
        price = handles.turbinePrice{2}*number1;
    case 4
        number = get(handles.noTurbine3,'String');
        number = str2num(number);
        number1 = fix(225/handles.turbineArea{1,3});
        number = number + number1;
        set(handles.noTurbine3,'String',num2str(number));
        windEnergyAdd = handles.averageWind{1,1}*number1*500;
        price = handles.turbinePrice{3}*number1;
    case 5
        number = get(handles.noTurbine4,'String');
        number = str2num(number);
        number1 = fix(225/handles.turbineArea{1,4});
        number = number + number1;
        set(handles.noTurbine4,'String',num2str(number));
        windEnergyAdd = handles.averageWind{1,1}*number1*500;
        price = handles.turbinePrice{4}*number1;
    case 6
        number = get(handles.noTurbine5,'String');
        number = str2num(number);
        number1 = fix(225/handles.turbineArea{1,5});
        number = number + number1;
        set(handles.noTurbine5,'String',num2str(number));
        windEnergyAdd = handles.averageWind{1,1}*number1*500;
        price = handles.turbinePrice{5}*number1;
    case 7
        number = get(handles.noTurbine6,'String');
        number = str2num(number);
        number1 = fix(225/handles.turbineArea{1,6});
        number = number + number1;
        set(handles.noTurbine6,'String',num2str(number));
        windEnergyAdd = handles.averageWind{1,1}*number1*500;
        price = handles.turbinePrice{6}*number1;
    case 8
        number = get(handles.noTurbine7,'String');
        number = str2num(number);
        number1 = fix(225/handles.turbineArea{1,7});
        number = number + number1;
        set(handles.noTurbine7,'String',num2str(number));
        windEnergyAdd = handles.averageWind{1,1}*number1*500;
        price = handles.turbinePrice{7}*number1;
    case 9
        number = get(handles.noTurbine8,'String');
        number = str2num(number);
        number1 = fix(225/handles.turbineArea{1,8});
        number = number + number1;
        set(handles.noTurbine8,'String',num2str(number));
        windEnergyAdd = handles.averageWind{1,1}*number1*500;
        price = handles.turbinePrice{8}*number1;
end
totalNumber = str2num(get(handles.totalWindTurbines,'String'));
totalNumber = totalNumber + number1;
set(handles.totalWindTurbines,'String',num2str(totalNumber));
windEnergy = str2num(get(handles.noWindEstimate,'String'));
windEnergy = round(windEnergy + windEnergyAdd);
set(handles.noWindEstimate,'String',num2str(windEnergy));
totalPrice = str2num(get(handles.totalPrice,'String'));
totalPrice = round(totalPrice + price);
set(handles.totalPrice,'String',num2str(totalPrice));
end
end
else
    msgbox('Error. Please try again. Hint: Did you click inside of the map?','Error','error')
end

imshow(handles.map)
guidata(hObject,handles)


 
% --- Executes during object creation, after setting all properties.
function turbinePopUp_CreateFcn(hObject, eventdata, handles)
% hObject    handle to turbinePopUp (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called
 
% Hint: popupmenu controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end
 
 
% --- Executes on button press in home.
function home_Callback(hObject, eventdata, handles)
% hObject    handle to home (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
  choice = questdlg('Are you sure you want to return home? You will lose your data if unsaved.', ... 
 'Warning', ... 
 'Yes','No','No'); 

switch choice
    case 'Yes'
        setappdata(0,'close',1);
        setappdata(0,'ReturnText',0);
        close(gui);
        guidata(gui_selectCity);
        gui_selectCity;
    case 'No'
        % Nothing happens
end
 
% --- Executes on button press in undo.
function undo_Callback(hObject, eventdata, handles)
% hObject    handle to undo (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
 
function loadData_Callback(hObject,eventdata,handles)
% hObject    handle to undo (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

%%
fileName = uigetfile('*.txt','Select file...');
% load image from .txt

b = dlmread(fileName);
c = reshape(b, handles.sizeC, handles.sizeR, 3);
handles.map = uint8(c);
imshow(handles.map,'Parent',handles.axes1);

%%
b= handles.b;
switch b
    case 1
        fileLoadData = fopen('allUserDataChicago.txt');
    case 2
        fileLoadData = fopen('allUserDataSanFrancisco.txt');
    case 3
        fileLoadData = fopen('allUserDataDenver.txt');
end
Data = fscanf(fileLoadData,'%d');
set(handles.noPanel1,'String',num2str(Data(1)));
set(handles.noPanel2,'String',num2str(Data(2)));
set(handles.noPanel3,'String',num2str(Data(3)));
set(handles.noPanel4,'String',num2str(Data(4)));
set(handles.noPanel5,'String',num2str(Data(5)));
set(handles.noPanel6,'String',num2str(Data(6)));
set(handles.noPanel7,'String',num2str(Data(7)));
set(handles.noPanel8,'String',num2str(Data(8)));
set(handles.noPanel9,'String',num2str(Data(9)));
set(handles.noPanel10,'String',num2str(Data(10)));
set(handles.totalSolarPanels,'String',num2str(Data(11)));
set(handles.noTurbine1,'String',num2str(Data(12)));
set(handles.noTurbine2,'String',num2str(Data(13)));
set(handles.noTurbine3,'String',num2str(Data(14)));
set(handles.noTurbine4,'String',num2str(Data(15)));
set(handles.noTurbine5,'String',num2str(Data(16)));
set(handles.noTurbine6,'String',num2str(Data(17)));
set(handles.noTurbine7,'String',num2str(Data(18)));
set(handles.noTurbine8,'String',num2str(Data(19)));
set(handles.totalWindTurbines,'String',num2str(Data(20)));
set(handles.noRemainingBoxes,'String',num2str(Data(21)));
set(handles.totalPrice,'String',num2str(Data(22)));
set(handles.noSolarEstimate,'String',num2str(Data(23)));
set(handles.noWindEstimate,'String',num2str(Data(24)));


guidata(hObject,handles);


 
% --- Executes on button press in help.
function help_Callback(hObject, eventdata, handles)
uiwait(msgbox('This is the guide on how to use the GUI. You have already selected your desired city. On the right hand side is a list of different models of solar panels and wind turbines that you will select from. Click on any of the options, and then drag your mouse within the map and click any box that you would like to place your selected panel/turbine in. The program will not let you click outside of the box. After you click a box in the map, the number of solar panels within that box (along with the corresponding color) will display on the left hand side. On the bottom, the total price and energy estimate will also update. A user can only click up to 20 boxes at one time. After you are done filling in the boxes with your desired panels or turbines, be sure to save your work pressing the "Save" button. If you want to clear the map, press "Clear All." In addition, if you have previous data that you would like to work with, press the "Load Data" button, which will only load previous .txt files that have been saved in your desired directory. If you have more questions or need further assistance, contact our group at ThreeMusketeers@ucdavis.edu. Thank you for using our graphical user interface. ','Help','help'));
% hObject    handle to help (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
 
 
% --- Executes on button press in save.
function save_Callback(hObject, eventdata, handles)
% hObject    handle to save (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
%% Save IMAGE into TEXTFILE




[picName, pathPic, userOption] = uiputfile('*.png','Save image (.png) as...');


if userOption ~=0
      
fileName = fullfile(pathPic, picName);
imwrite(handles.map,fileName);

%%
[fileName_image, pathFile] = uiputfile('*.txt','Save current image in text file (** You will need this if you want to load data later**)');
a = imread(fileName); %save image
[handles.sizeC, handles.sizeR, color]=size(a);
dlmwrite(fileName_image, a, 'delimiter', ',');
%%
b= handles.b;
switch b
    case 1
        fID_userData = fopen('allUserDataChicago.txt','wt');
    case 2
        fID_userData = fopen('allUserDataSanFrancisco.txt','wt');
    case 3
        fID_userData = fopen('allUserDataDenver.txt','wt');
end
% Solar Panels
user_solarPanels = [str2num(get(handles.noPanel1,'String')), str2num(get(handles.noPanel2,'String')), str2num(get(handles.noPanel3,'String')), str2num(get(handles.noPanel4,'String')), str2num(get(handles.noPanel5,'String')), str2num(get(handles.noPanel6,'String')), str2num(get(handles.noPanel7,'String')), str2num(get(handles.noPanel8,'String')), str2num(get(handles.noPanel9,'String')), str2num(get(handles.noPanel10,'String'))];
for x = 1:length(user_solarPanels)
    fprintf(fID_userData,'%d\n',user_solarPanels(x));
end
% Total Solar Panels
fprintf(fID_userData,'%d\n',str2num(get(handles.totalSolarPanels,'String')));
% Wind Turbines
user_windTurbines = [str2num(get(handles.noTurbine1,'String')), str2num(get(handles.noTurbine2,'String')), str2num(get(handles.noTurbine3,'String')), str2num(get(handles.noTurbine4,'String')), str2num(get(handles.noTurbine5,'String')), str2num(get(handles.noTurbine6,'String')), str2num(get(handles.noTurbine7,'String')), str2num(get(handles.noTurbine8,'String'))];
for n = 1:length(user_windTurbines)
    fprintf(fID_userData,'%d\n',user_windTurbines(n));
end
% Total Wind Turbines
fprintf(fID_userData,'%d\n',str2num(get(handles.totalWindTurbines,'String')));
% Remaining Boxes
fprintf(fID_userData,'%d\n',str2num(get(handles.noRemainingBoxes,'String')));

% Total Price
totPriceNum = str2num(get(handles.totalPrice,'String'));
roundPriceNum = round(totPriceNum);
fprintf(fID_userData,'%d\n',roundPriceNum);

% Solar Energy Estimate
totSolarEst = str2num(get(handles.noSolarEstimate,'String'));
roundSolarEst = round(totSolarEst);
fprintf(fID_userData,'%d\n',roundSolarEst);

% Wind Energy Estimate
fprintf(fID_userData,'%d\n',str2num(get(handles.noWindEstimate,'String')));


fclose(fID_userData);



%%
[txtName, pathTxt] = uiputfile('*.txt','Save your bill of materials (receipt) in .txt as...');

fileName2=fullfile(pathTxt,txtName);

fileID=fopen(fileName2,'wt');

fprintf(fileID,'Bill of Materials: %s\n',handles.city{1,1});
fprintf(fileID,'----------------------------------------------------------------\n');

fprintf(fileID,'Amount of solar panels selected:\n');
solarPanels = {'Sunforce 500', 'Sunforce 39810', 'Instapark SPCC-5W', 'Instapark SP-100W', 'Instapark SPCC-30W', 'Instapark SP-10W', 'Ramsond 100SP', 'Epcom WK50-12', 'Sun Power E18', 'Sun Power T5'}';
solarPanels(:,2) = {get(handles.noPanel1,'String'), get(handles.noPanel2,'String'), get(handles.noPanel3,'String'), get(handles.noPanel4,'String'), get(handles.noPanel5,'String'), get(handles.noPanel6,'String'), get(handles.noPanel7,'String'), get(handles.noPanel8,'String'), get(handles.noPanel9,'String'), get(handles.noPanel10,'String')}';
for x = 1:length(solarPanels)
fprintf(fileID,'%s\n',solarPanels{x,:});
end
fprintf(fileID,'----------------------------------------------------------------\n');

fprintf(fileID,'Amount of wind turbines selected:\n');
windTurbines = {'Windmax HY 1000-5', 'Windmax HY400', 'GudCraft WG400', 'WG700', 'All Power America APWT400A', 'Sunforce 45444', 'Sunforce 44444', 'WindyNation WCK-750'}';
windTurbines(:,2) = {get(handles.noTurbine1,'String'), get(handles.noTurbine2,'String'), get(handles.noTurbine3,'String'), get(handles.noTurbine4,'String'), get(handles.noTurbine5,'String'), get(handles.noTurbine6,'String'), get(handles.noTurbine7,'String'), get(handles.noTurbine8,'String')}';

for x = 1:length(windTurbines)
fprintf(fileID,'%s\n',windTurbines{x,:});
end

fprintf(fileID,'----------------------------------------------------------------\n');

fprintf(fileID,'Total Cost:\n');

fprintf(fileID,'$%s',get(handles.totalPrice,'String'));
fclose(fileID);
%%

msgbox('Data successfully saved! (Data includes your bill of materials and modified image of the city)');
else
  msgbox('Operation canceled.')  ;
end

guidata(hObject, handles);


% --- Executes on button press in clearAll.
function clearAll_Callback(hObject, eventdata, handles)
% hObject    handle to clearAll (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
 choice = questdlg('Are you sure you want to clear the entire image?', ... 
 'Warning', ... 
 'Yes','No','No'); 

switch choice
    case 'Yes'
        b= handles.b;
switch b
    case 1
        handles.map=imread('Chicago.bmp'); %% Using sample map
        size_of_box=10; %% 10x10
        % Create boxes on the map.
        handles.map(1:size_of_box:end,:,:)=0;
        handles.map(:,1:size_of_box:end,:)=0;
        imshow(handles.map);
    case 2
        handles.map=imread('SanFrancisco.bmp'); %% Using sample map
        size_of_box=10; %% 10x10
        % Create boxes on the map.
        handles.map(1:size_of_box:end,:,:)=0;
        handles.map(:,1:size_of_box:end,:)=0;
        imshow(handles.map);
    case 3
        handles.map=imread('Denver.bmp'); %% Using sample map
        size_of_box=10; %% 10x10
        % Create boxes on the map.
        handles.map(1:size_of_box:end,:,:)=0;
        handles.map(:,1:size_of_box:end,:)=0;
        imshow(handles.map);
end

        
set(handles.noPanel1,'String','0');
set(handles.noPanel2,'String','0');
set(handles.noPanel3,'String','0');
set(handles.noPanel4,'String','0');
set(handles.noPanel5,'String','0');
set(handles.noPanel6,'String','0');
set(handles.noPanel7,'String','0');
set(handles.noPanel8,'String','0');
set(handles.noPanel9,'String','0');
set(handles.noPanel10,'String','0');
set(handles.totalSolarPanels,'String','0');

set(handles.noTurbine1,'String','0');
set(handles.noTurbine2,'String','0');
set(handles.noTurbine3,'String','0');
set(handles.noTurbine4,'String','0');
set(handles.noTurbine5,'String','0');
set(handles.noTurbine6,'String','0');
set(handles.noTurbine7,'String','0');
set(handles.noTurbine8,'String','0');
set(handles.totalWindTurbines,'String','0');

set(handles.noRemainingBoxes,'String','20');
set(handles.totalPrice,'String','0');

set(handles.noSolarEstimate,'String','0');
set(handles.noWindEstimate,'String','0');

    case 'No'
end
 
guidata(hObject,handles);


% --- Executes on button press in remove.
function remove_Callback(hObject, eventdata, handles)
% hObject    handle to remove (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
[xCoordinate, yCoordinate] = ginput(1);
 %%

% Updates the number of remaining boxes
if ((xCoordinate <790) & (yCoordinate <460))&((xCoordinate >=0)&(yCoordinate >=0))
    
%%Color
 
size_of_box=10;
xCoordinate = fix(xCoordinate);
yCoordinate = fix(yCoordinate);
mapgrid=handles.map;
mapgrid(:)=255;
mapgrid(1:size_of_box:end,:)=0;
mapgrid(:,1:size_of_box:end)=0;

% Find where the top horizontal axis is.
tmp=0;
i=fix(yCoordinate-size_of_box);
if i<=0
    i=0;
elseif mapgrid(i,xCoordinate)==0
    uiwait(msgbox('Please click within the map','Error','error'));
    tmp=tmp+1;
else
while mapgrid(i,xCoordinate)~=0
    i=i+1;
end
end
% Find where the left vertical axis of the box
ii=fix(xCoordinate-size_of_box);
if ii<=0
    ii=0;
else
while mapgrid(yCoordinate,ii)~=0 
    ii=ii+1;
end
end
if tmp==0
if  handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==handles.originalMap((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==handles.originalMap((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==handles.originalMap((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)
     uiwait(msgbox('No panels or turbines in the box','Error','error'));
else
% Change the color of the box and display the new map.
delete =0;
if (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) == 255)
        number = get(handles.noPanel1,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,1});
        number = number - number1;
        set(handles.noPanel1,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,1});
        price = handles.panelPrice{1}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 100)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 149)& (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==237)
        number = get(handles.noPanel2,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,2});
        number = number - number1;
        set(handles.noPanel2,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,2});
        price = handles.panelPrice{2}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 0)& (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==128)
        number = get(handles.noPanel3,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,3});
        number = number - number1;
        set(handles.noPanel3,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,3});
        price = handles.panelPrice{3}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 30)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 144)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==255
        number = get(handles.noPanel4,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,4});
        number = number - number1;
        set(handles.noPanel4,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,4});
        price = handles.panelPrice{4}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 206)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==209
        number = get(handles.noPanel5,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,5});
        number = number - number1;
        set(handles.noPanel5,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,5});
        price = handles.panelPrice{5}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 150)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 205)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==205
        number = get(handles.noPanel6,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,6});
        number = number - number1;
        set(handles.noPanel6,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,6});
        price = handles.panelPrice{6}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 0)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 199)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==140
        number = get(handles.noPanel7,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,7});
        number = number - number1;
        set(handles.noPanel7,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,7});
        price = handles.panelPrice{7}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 69)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 139)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==116
        number = get(handles.noPanel8,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,8});
        number = number - number1;
        set(handles.noPanel8,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,8});
        price = handles.panelPrice{8}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 148)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 188)& handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==143
        number = get(handles.noPanel9,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,9});
        number = number - number1;
        set(handles.noPanel9,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,9});
        price = handles.panelPrice{9}*number1;
        delete = 1;
elseif (handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1) == 118)&(handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2) == 238)&handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3) ==0
        number = get(handles.noPanel10,'String');
        number = str2num(number);
        number1 = fix(225/handles.panelArea{1,10});
        number = number - number1;
        set(handles.noPanel10,'String',num2str(number));
        solarEnergyDeduct = round(handles.averageSolar{1,1}*number1*handles.panelArea{1,10});
        price = handles.panelPrice{10}*number1;
        delete = 1;
end
if delete == 1
totalNumber = str2num(get(handles.totalSolarPanels,'String'));
totalNumber = totalNumber - number1;
set(handles.totalSolarPanels,'String',num2str(totalNumber));
solarEnergy = str2num(get(handles.noSolarEstimate,'String'));
solarEnergy = round(solarEnergy - solarEnergyDeduct);
set(handles.noSolarEstimate,'String',num2str(solarEnergy));
totalPrice = str2num(get(handles.totalPrice,'String'));
totalPrice = round(totalPrice - price);
set(handles.totalPrice,'String',num2str(totalPrice));
oldRemBoxesString = get(handles.noRemainingBoxes,'String');
oldRemBoxes = str2num(oldRemBoxesString);
updateRemBoxes = oldRemBoxes + 1;
set(handles.noRemainingBoxes,'String',num2str(updateRemBoxes))
end
delete = 0;
if handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==0
    number = get(handles.noTurbine1,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,1});
    number = number - number1;
    set(handles.noTurbine1,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{1}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==205 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==173 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==0
    number = get(handles.noTurbine2,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,2});
    number = number - number1;
    set(handles.noTurbine2,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{2}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==139 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==117 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==0
    number = get(handles.noTurbine3,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,3});
    number = number - number1;
    set(handles.noTurbine3,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{3}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==139 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==69 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==19
    number = get(handles.noTurbine4,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,4});
    number = number - number1;
    set(handles.noTurbine4,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{4}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==97 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==3
    number = get(handles.noTurbine5,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,5});
    number = number - number1;
    set(handles.noTurbine5,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{5}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==130 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==71
    number = get(handles.noTurbine6,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,6});
    number = number - number1;
    set(handles.noTurbine6,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{6}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==255 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==69 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==0
    number = get(handles.noTurbine7,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,7});
    number = number - number1;
    set(handles.noTurbine7,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{7}*number1;
    delete = 1;
elseif handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)==205 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)==85 & handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)==85
    number = get(handles.noTurbine8,'String');
    number = str2num(number);
    number1 = fix(225/handles.turbineArea{1,8});
    number = number - number1;
    set(handles.noTurbine8,'String',num2str(number));
    windEnergyDeduct = handles.averageWind{1,1}*number1*500;
    price = handles.turbinePrice{8}*number1;
    delete = 1;
end
if delete ==1
    totalNumber = str2num(get(handles.totalWindTurbines,'String'));
    totalNumber = totalNumber - number1;
    set(handles.totalWindTurbines,'String',num2str(totalNumber));
    windEnergy = str2num(get(handles.noWindEstimate,'String'));
    windEnergy = round(windEnergy - windEnergyDeduct);
    set(handles.noWindEstimate,'String',num2str(windEnergy));
    totalPrice = str2num(get(handles.totalPrice,'String'));
    totalPrice = round(totalPrice - price);
    set(handles.totalPrice,'String',num2str(totalPrice));
    oldRemBoxesString = get(handles.noRemainingBoxes,'String');
    oldRemBoxes = str2num(oldRemBoxesString);
    updateRemBoxes = oldRemBoxes + 1;
    set(handles.noRemainingBoxes,'String',num2str(updateRemBoxes))
end

    handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1)=handles.originalMap((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),1); %% Set the color
    handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2)=handles.originalMap((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),2);
    handles.map((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3)=handles.originalMap((i+1):(i+size_of_box-1),(ii+1):(ii+size_of_box-1),3);
end
end
else
     msgbox('Error. Please try again. Hint: Did you click inside of the map?','Error','error')
end
imshow(handles.map);

guidata(hObject,handles);





