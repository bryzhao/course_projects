function varargout = gui_selectCity(varargin)
% GUI_SELECTCITY MATLAB code for gui_selectCity.fig
%      GUI_SELECTCITY, by itself, creates a new GUI_SELECTCITY or raises the existing
%      singleton*.
%
%      H = GUI_SELECTCITY returns the handle to a new GUI_SELECTCITY or the handle to
%      the existing singleton*.
%
%      GUI_SELECTCITY('CALLBACK',hObject,eventData,handles,...) calls the local
%      function named CALLBACK in GUI_SELECTCITY.M with the given input arguments.
%
%      GUI_SELECTCITY('Property','Value',...) creates a new GUI_SELECTCITY or raises the
%      existing singleton*.  Starting from the left, property value pairs are
%      applied to the GUI before gui_selectCity_OpeningFcn gets called.  An
%      unrecognized property name or invalid value makes property application
%      stop.  All inputs are passed to gui_selectCity_OpeningFcn via varargin.
%
%      *See GUI Options on GUIDE's Tools menu.  Choose "GUI allows only one
%      instance to run (singleton)".
%
% See also: GUIDE, GUIDATA, GUIHANDLES

% Edit the above text to modify the response to help gui_selectCity

% Last Modified by GUIDE v2.5 16-Mar-2014 12:22:29

% Begin initialization code - DO NOT EDIT
gui_Singleton = 1;
gui_State = struct('gui_Name',       mfilename, ...
                   'gui_Singleton',  gui_Singleton, ...
                   'gui_OpeningFcn', @gui_selectCity_OpeningFcn, ...
                   'gui_OutputFcn',  @gui_selectCity_OutputFcn, ...
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


% --- Executes just before gui_selectCity is made visible.
function gui_selectCity_OpeningFcn(hObject, eventdata, handles, varargin)
% This function has no output args, see OutputFcn.
% hObject    handle to figure
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
% varargin   command line arguments to gui_selectCity (see VARARGIN)

% Choose default command line output for gui_selectCity
handles.output = hObject;
c = 0;
setappdata(0,'close',c);

axes(handles.a);
image1 = imread('chicagoPic.png');
imshow(image1);

axes(handles.b);
image2 = imread('sfPic.png');
imshow(image2);

axes(handles.c);
image3 = imread('denverPic.png');
imshow(image3);

% Update handles structure
guidata(hObject, handles);



% UIWAIT makes gui_selectCity wait for user response (see UIRESUME)
% uiwait(handles.figure1);


% --- Outputs from this function are returned to the command line.
function varargout = gui_selectCity_OutputFcn(hObject, eventdata, handles) 
% varargout  cell array for returning output args (see VARARGOUT);
% hObject    handle to figure
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Get default command line output from handles structure
varargout{1} = handles.output;


% --- Executes on button press in selectChicago.
function selectChicago_Callback(hObject, eventdata, handles)
% hObject    handle to selectChicago (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
a = 1;
setappdata(0,'ReturnText',a);
close(gui_selectCity);
gui;





% --- Executes on button press in selectSanFrancisco.
function selectSanFrancisco_Callback(hObject, eventdata, handles)
% hObject    handle to selectSanFrancisco (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
a = 2;
setappdata(0,'ReturnText',a);
close(gui_selectCity);
gui;




% --- Executes on button press in selectDenver.
function selectDenver_Callback(hObject, eventdata, handles)
% hObject    handle to selectDenver (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
a = 3;
setappdata(0,'ReturnText',a);
close(gui_selectCity);
gui;


% --- Executes during object creation, after setting all properties.
function a_CreateFcn(hObject, eventdata, handles)
% hObject    handle to a (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: place code in OpeningFcn to populate a


% --- Executes during object creation, after setting all properties.
function b_CreateFcn(hObject, eventdata, handles)
% hObject    handle to b (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: place code in OpeningFcn to populate b


% --- Executes during object creation, after setting all properties.
function c_CreateFcn(hObject, eventdata, handles)
% hObject    handle to c (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: place code in OpeningFcn to populate c
