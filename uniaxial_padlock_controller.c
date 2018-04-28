    /*********************************************************/
    /******************* FREE-TIME SYSTEM ********************/
    /*********************************************************/
    /******************** EME154 UCDAVIS *********************/
    /*********************************************************/
    # include <8052.h>
    # include <stdio.h>
    # include <stdlib.h>
    # include <MORPH.h>
 
    /**** FUNCTION PROTOTYPES ********************************/
    void OCSfunction();
    void MOSfunction(char selection);
    void ACSfunction();
 
    /*********************************************************/
    /***** START OF FTS **************************************/
    /*********************************************************/
 
    /***** REMARKS SECTION ***********************************
    FTS - Free time System
    INZ - Initialization
    DIG - Diagnostics
    ERH - Error Treatment Supervisor
    MSS - Machine Status Scan Supervisor
    MCS - Mode Control Supervisor
    ACS - Automatic Control Supervisor
    MOS - Manual Operation Supervisor
    PGS - Programming Supervisor
    MSD - Machine Setup Data Supervisor
    OCS - Output Control Supervisor
    ***** END OF REMARKS SECTION *****************************/
 
 
    /*********************************************************/
    /***** FTS PROGRAM SECTION *******************************/
    /*********************************************************/
 
    /***** START OF INZ **************************************/
    unsigned char OMD, ERR1, FFRA, samplingTime, dialPositions;
    unsigned int feedRate;       //set speed in RPM
    unsigned int encoderResolution;        //set encoder resolution
 
    __data __at (0x52) unsigned int speed;
    __data __at (0x56) unsigned long distance;
    __data __at (0x23) unsigned char counter;
 
 
    //motion register:: XGO(0th bit), INPOS(1st bit), direction(4th bit)
    __data __at (0x21) unsigned char motionRegister;
 
    char string[5];
    char input[2]; // input for dial positions
    char *machineMessage, *menuTitle, *action1, *action2, *action3, *action4, *action5, *action6, *action7, *action8, *action9;
 
    /***** Initilization function ****************************/
    void INZfunction(){
    printMode = 0; //Output is routed to the MPS Console
    OMD=0;// REM Operation Mode is Idle
    ERR1=0;//REM Level 1 Error Flag of System
    FFRA=1;//REM SET First Run Flag
    P1 = 0x00;//Solenoid port initialization
	counter = 0; //timer counter initialization 5_16_2014
    clrUDCounter();
 
    motionRegister = 0x00; //CW direction, !inPOS, !XGO
    encoderResolution = 2048*4;
    dialPositions = 40;
    samplingTime = 10; //sampling time is 10ms
    feedRate = 25;//50; //set motor speed in RPM
    speed = (feedRate*samplingTime*(float)encoderResolution)/60000+0.5;//set speed
    machineMessage = " ";
 
    // obtain initial dial position for tracking
    printf("Welcome! Please enter initial dial position: \n");
    gets(input);
    while ( (input[1]<48 || input[1]>57) || (input[0]<48 || input[0]>51) ){
        printf("Please input a valid initial dial number in format of XX: \n");
        gets(input);
    }
    initialPosition = atoi(input);
    manualPos = initialPosition;
        
    //initialize Real Time System
    __asm
    lcall 0x9F00
    __endasm;
 
    }
    /***** END OF INZ ****************************************/
 
    /***** START of MESSAGE AND CONSTANTS DEFINITION *********/
    void printHeaderAndMenu(){
    clrPC();
    printf("EME 154 Mechatronics\n");
    printf("Welcome to the Automatic Padlock Opener!\n");
    printf("- Bryan Zhao and Erik Braun\n\n");
    printf("****************************************\n");
    printf("%s\n", menuTitle);
    printf("****************************************\n\n");
    printf("%s\n", action1);
    printf("%s\n", action2);
    printf("%s\n", action3);
    printf("%s\n", action4);
    printf("%s\n", action5);
    printf("%s\n", action6);
    printf("%s\n", action7);
    printf("%s\n", action8);
    printf("9. Exit\n\n\n");
    }

    void setIdleMenu(){
    menuTitle = " OPERATION MENU";
    action1 = "Press 1: Manual Operation";
    action2 = "Press 2: Automatic Operation";
    action3 = "";
    action4 = "";
    action5 = "";
    action6 = "";
    action7 = "";
    action8 = "";
    }
 
    void setManualMenu(){
    menuTitle = " MANUAL OPERATION MENU";
    action1 = "Press 1: Move One Tick CCW";
    action2 = "Press 2: Move One Tick CW";
    action3 = "Press 3: Move 5 Ticks CCW";
    action4 = "Press 4: Move 5 Ticks CW";
    action5 = "Press 5: Move One full rotation CCW";
    action6 = "Press 6: Move One full rotation CW";
    action7 = "Press 7: Move half turn CCW";
    action8 = "Press 8: Move half turn CW";
    }

    void setAutomaticMenu(){
    menuTitle = " AUTOMATIC MODE";
    action1 = "Welcome to the automatic mode!";
    action2 = "Please press space to begin.";   // get rid of these?? agreed, or we can use them as filler text?
    action3 = "After pressing space, please follow the prompts, and ";
    action4 = "enter 3 combination numbers that are between 0-39 as such: 02, 21, 09";
    action5 = "rather than 2, 21, 9. Two digits are required for each numerical entry.";
    action6 = "";
    action7 = "";
    action8 = "";
    }


    void moveServo(char ticks, char direction){
    distance = (float)ticks*(float)(encoderResolution / dialPositions);
    //the 0x01 is the XGO bit and 0x10 is the directional bit
    //direction 0 & 1 equate CW & CCW, respectively
    motionRegister = 0x01+0x10*direction;
    }
 
    int getABSposition(){
    __data __at (0x60) unsigned long position;
    return position;
    }
    /***** END of MESSAGE AND CONSTANTS DEFINITION ***********/
 
    /***** START OF DIG **************************************/
    int diagnostics(){
    if(ERR1) {
    return 0;//for error
    }else{
    return 1;//successfull diagnostics
    }
    }
    /***** END OF DIG ****************************************/
 
    // removed ERH, not utilizing it in our FTS
 
    /***** START OF MSS **************************************/
    void MSSfunction(){
    char selection;
    selection = key();

      
    if (FFRA) OCSfunction();     //skip this for the first run
    //Scan Keyboard for input
    switch (selection){
    case '1':         //this key will...
    if (OMD == 1){    //in manual mode
    MOSfunction(selection);
    } else if (OMD == 0){   //in idle mode
    OMD = 1;
    setManualMenu();
    machineMessage = "Manual Mode Accepted";
    printHeaderAndMenu();
    }
    break;
    case '2': //this key will...
    if (OMD == 1){ // in manual mode, move 1 tick CW
    MOSfunction(selection);
    } else if (OMD == 0){//in idle mode, go to automatic mode
    OMD = 2;
    setAutomaticMenu();
    printHeaderAndMenu();
    }
    break;
    case '3': // this key will...
      if (OMD == 1){ // in manual mode, move 1 tick CCW
        MOSfunction(selection);
      }
    break;
    case '4': // this key will...
      if (OMD == 1){ // in manual mode, move 5 ticks CW
        MOSfunction(selection);
      }
    break;
    case '5': //this key will...
        if (OMD == 1){ // in manual mode, move 40 ticks CCW (full rotation)
            MOSfunction(selection);
    	}
    break;
    case '6': // this key will...
      if (OMD == 1){ // in manual mode, move 40 ticks CW (full rotation)
        MOSfunction(selection);
      }
    break;
    case '7': // this key will...
      if (OMD == 1){ // in manual mode, move 20 dial ticks CCW (half rotation)
        MOSfunction(selection);
      }
    break;
    case '8': // this key will...
      if (OMD == 1){ // in manual mode, move 20 dial ticks CW (half rotation)
        MOSfunction(selection);
      }
    break;
    case '9': 
        if (OMD == 0){
            OMD = 5;
            machineMessage = "Exit Command Excepted";
        } else {
            setIdleMenu();
            printHeaderAndMenu();
            OMD = 0;
        }
        break;
    }//end switch
        
    }//end of MSSfunction

    /***** END OF MSS ****************************************/
 
    /***** START OF MCS **************************************/
    void MCSfunction(){
    if (OMD==1) MOSfunction(0);
    if (OMD==2) ACSfunction();
 
    }
    /***** END OF MCS ****************************************/
 
    /***** START OF MOS **************************************/
    void MOSfunction(char selection){
    if (selection){
    	switch (selection){
    	case '1': //this key moves the dial 1 tick CCW
    	moveServo(1,1); //move 1 tick CCW
    	break;
    	case '2': //this key moves the dial 1 tick CW
    	moveServo(1,0); //zero is CW
    	break;
    	case '3': // this key moves dial 5 ticks CCW
   	 	moveServo(5,1);
    	break;
  		case '4': // key moves dial 5 ticks CW
    	moveServo(5,0);
    	break; 
    	case '5': // key moves dial 40 ticks CCW
    	moveServo(40,1);
    	break;
    	case '6': // key moves dial 40 ticks CW
    	moveServo(40,0);
    	break;
      case '7':
      moveServo(20,1); // key moves dial 20 ticks CCW - half rotation
      break;
      case '8':
      moveServo(20,0); // key moves dial 20 ticks CW - half rotation
      break;
    	case '9': // exit case to return to operation menu
    	break;
    	}
     }
    }
    /***** END OF MOS ****************************************/
 
 
    /***** START OF ACS **************************************/
    void ACSfunction(){
   
    //prompts for user input before starting ACS 
    int comb1, comb2, comb3, initial, first, second, third; // combination values
    int a = 0; // keyboard input
    
    printf("Press Space to begin Automatic Mode:\n");
    while (a != 32){ // waits until space bar is pressed on keyboard
        a = key();
      }
    counter = 0;
      
    // once space bar is pressed, prompt user for combination inputs
    printf("Enter initial dial position between 00-39 in the form of XX: \n");
    gets(input);
    while ((input[1]<48 || input[1]>57) || (input[0]<48 || input[0]>51)){           
        printf("Please input a valid dial number: \n");
      	gets(input);	
    } // ERH
    initial = atoi(input); // store as integer
       
    printf("Enter first combination number between 00-39: \n");
    gets(input);    
    while ((input[1]<48 || input[1]>57) || (input[0]<48 || input[0]>51)){      
        printf("Please input a valid dial number: \n");
      	gets(input);
    } // ERH
    comb1 = atoi(input);
   
   	// underflow, overflow control statements to determine amount of ticks needed to move to position
    if (comb1 < initial){
        first = initial - comb1;   
        printf("The first number of offset ticks is: %d \n", first);
    }
    else {
        first = (40 - comb1) + initial;
        printf("The first number of offset ticks is: %d \n", first);
    } // end of first
      
    printf("Enter second combination number between 00-39: \n");
    gets(input);
    while ((input[1]<48 || input[1]>57) || (input[0]<48 || input[0]>51)){           
            printf("Please input a valid dial number: \n");
      	gets(input);
    } // ERH
    comb2 = atoi(input); // store as integer
    
    if (comb2 < comb1){ // underflow, overflow control
    	second = (40 - comb1) + comb2;
    	printf("The second number of offset ticks is: %d \n", second);
    }
    else {
        second = comb2 - comb1;
    	printf("The second number of offset ticks is: %d \n", second);
    } // end of second
  
    printf("Enter third combination number between 00-39: \n");
    gets(input);
    while ((input[1]<48 || input[1]>57) || (input[0]<48 || input[0]>51)){              
            printf("Please input a valid dial number: \n");
      	gets(input);
    } // ERH
    comb3 = atoi(input); // store as integer
    
    if (comb3 < comb2){ // underflow, overflow control
    	third = comb2-comb3;
    }
    else {
    	third = (40-comb3)+comb2;
    }  

	// now, begin moving motor!
    moveServo(80,1); // 2 full rotations
    
    // while not in position
    while (!(motionRegister & 02));
    printf("Moving to the first number... \n");
    printf("Initial position entered: %d \n" ,initial);
    printf("Value of first: %d \n", first);
    printf("Stored value of 1st combination number: %d \n", comb1);
    moveServo(first,1);
      
    while (!(motionRegister & 02));  
    moveServo(40,0);  // one full rotation CW  
      
    while (!(motionRegister & 02));
    printf("Moving to the second number... \n");
    printf("Value of second: %d \n", second);
    printf("Stored value of 2nd combination number: %d \n", comb2);
    moveServo(second,0);
      
    while (!(motionRegister & 02));
    printf("Moving to third number... \n");
    printf("Value of third: %d \n", third);
    printf("Stored value of 3rd combination number: %d \n", comb3);
    moveServo(actualThird,1);
      
    while (!(motionRegister & 02)); // finished with rotation, open solenoid
    //pop the lock with the solenoid connected to port 1
    printf("Press enter to release solenoid! \n");
    P1 = 0xFF;
	printf("Time elapsed in ms: %u", 10*counter);
    gets(string);
    P1 = 0x00;
 
    // exit automatic mode after completion!
    setIdleMenu();
    printHeaderAndMenu();
    OMD = 0;
    }
    /***** END OF ACS ****************************************/

 
    /***** Function gets mode name from OMD ******************/
    char *getCurrentMode(){
    switch (OMD){
    case 1:
    return "Manual Mode";
    case 2:
    return "Automatic Mode";
    case 5:
    return "System Turned Off";
    default:
    return "Idle Mode";
    }
    }
 
    char *getServoStatus(){
    if (motionRegister & 02){
    return "Stopped ";
    } else{
    return "Moving... ";
    }
    }
 
    /***** START OF OCS **************************************/
    void OCSfunction(){
    if(FFRA) {
    FFRA = 0;//Clear first run flag
    setIdleMenu();
    printHeaderAndMenu();
    }else{
    setCur(0,13);
    printf("%s \n", machineMessage);
    printf("Servo Status: %s     %u         \n", getServoStatus(), getABSposition());
    printf("Current Mode: %s\n\n", getCurrentMode());
    }
    }
    /***** END OF OCS ****************************************/
 
 
    /***** START OF MAIN *************************************/
    void main(){
 
    INZfunction();
 
    while(OMD != 5 && !microButtons){
    //run diagnostics
    if (diagnostics()){
    //if diagnostics succeed
    MSSfunction();
    MCSfunction();
    OCSfunction();
    }else{
    //go to ERH if diagnostics fail
    ERHfunction();
    OCSfunction();
    }
    }
    }
    /***** END OF MAIN ***************************************/
 
