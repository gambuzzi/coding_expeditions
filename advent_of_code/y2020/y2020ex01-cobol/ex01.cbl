  IDENTIFICATION DIVISION.                                        
  PROGRAM-ID. AOC_Y2020_EX01.                                           
  ENVIRONMENT DIVISION.                                           
  INPUT-OUTPUT SECTION.                                            
  FILE-CONTROL.                                                
      SELECT FILE1 ASSIGN "input.txt"
      ORGANIZATION IS LINE SEQUENTIAL                                   
      ACCESS MODE IS SEQUENTIAL                                    
      FILE STATUS IS WS-FS.                                       
  DATA DIVISION.                                                  
  FILE SECTION.                                                   
      FD  FILE1.                                                       
      01 STD-NO         PIC 9(05).                               
  WORKING-STORAGE SECTION.                                         
  77 WS-FS               PIC 9(02).                                
  01 WS-EOF-SW           PIC X(01) VALUE 'N'.                     
     88 EOF-SW           VALUE 'Y'.                                
     88 NOT-EOF-SW       VALUE 'N'.               
  01  LS-DYNAMIC-TBL-REC.
    03  LS-DYNAMIC-NBR-ENTRIES PIC  9(008) COMP.
    03  LS-DYNAMIC-ENTRIES     OCCURS 1 TO 1000 TIMES
                               DEPENDING ON LS-DYNAMIC-NBR-ENTRIES
                               INDEXED BY X-LS-DE, X-LS-DE-MAX, J, K
                               PIC  9(5).                 

  01  RES                PIC 9(10).
  01  RES2                PIC 9(10).
  01  RET                 PIC S9(10)9.
  01  VIS                 PIC Z(10)9.

  PROCEDURE DIVISION.  
  MAIN-PARA.                                            
      *> DISPLAY 'SEQUENTIAL FILE READING...'.                        
      OPEN INPUT FILE1.                                            
      PERFORM UNTIL EOF-SW                                        
         READ FILE1 INTO LS-DYNAMIC-ENTRIES(X-LS-DE)                                           
         AT END 
             MOVE 'Y' TO WS-EOF-SW                              
         NOT AT END 
             ADD 1 TO X-LS-DE
             ADD 1 TO X-LS-DE-MAX
         END-READ
      END-PERFORM.                                                 
      
      *> PART1

      MOVE 1 TO X-LS-DE
      MOVE -1 TO RET

      PERFORM UNTIL X-LS-DE >= X-LS-DE-MAX
          MOVE X-LS-DE TO J
          ADD 1 TO J
          PERFORM UNTIL J >= X-LS-DE-MAX
              ADD LS-DYNAMIC-ENTRIES(X-LS-DE) TO LS-DYNAMIC-ENTRIES(J) GIVING RES
              IF RES IS = 2020 THEN
                 MULTIPLY LS-DYNAMIC-ENTRIES(X-LS-DE) BY LS-DYNAMIC-ENTRIES(J) GIVING RET
                 MOVE RET TO VIS
                 DISPLAY 'PART1 -> ' VIS
                 EXIT PERFORM
              END-IF
              ADD 1 TO J
          END-PERFORM
          IF RET IS NOT = -1 THEN
              EXIT PERFORM
          END-IF

          ADD 1 TO X-LS-DE
      END-PERFORM

      *> PART2

      MOVE 1 TO X-LS-DE
      MOVE -1 TO RET

      PERFORM UNTIL X-LS-DE >= X-LS-DE-MAX
          MOVE X-LS-DE TO J
          ADD 1 TO J
          PERFORM UNTIL J >= X-LS-DE-MAX
              ADD LS-DYNAMIC-ENTRIES(X-LS-DE) TO LS-DYNAMIC-ENTRIES(J) GIVING RES
              MOVE J TO K
              ADD 1 TO K
              PERFORM UNTIL K >= X-LS-DE-MAX
                     ADD LS-DYNAMIC-ENTRIES(K) TO RES GIVING RES2
                     IF RES2 IS = 2020 THEN
                        MULTIPLY LS-DYNAMIC-ENTRIES(X-LS-DE) BY LS-DYNAMIC-ENTRIES(J) GIVING RET
                        MULTIPLY LS-DYNAMIC-ENTRIES(K) BY RET
                        MOVE RET TO VIS
                        DISPLAY 'PART2 -> ' VIS
                        EXIT PERFORM
                     END-IF
              ADD 1 TO K
              END-PERFORM
          IF RET IS NOT = -1 THEN
               EXIT PERFORM
          END-IF
          ADD 1 TO J
          END-PERFORM
          IF RET IS NOT = -1 THEN
              EXIT PERFORM
          END-IF

          ADD 1 TO X-LS-DE
      END-PERFORM

      CLOSE FILE1.                                                 
      STOP RUN.              