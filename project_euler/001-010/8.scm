; Solved: 18.12.17
; Changes:
;  * implement string-bytes method
;  * implement filter
; 29.12.17, Refactor to use chunks & product methods
; 11.1.18, Refactor to use ~> 

 (~> "project_euler/001-010/8.txt"
     file-read
     string->chars
     (select char-numeric?)
     (map char->digit)
     (chunks 13)
     (reduce &(max (product &1) &2) 0)
     (println "Solution: "))
