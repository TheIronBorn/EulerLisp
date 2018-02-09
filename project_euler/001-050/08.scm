; Solved: 18.12.2017

 (~> "project_euler/input-files/8.txt"
     file-read
     string->chars
     (select char-numeric?)
     (map char->digit)
     (chunks 13)
     (reduce &(max (product &1) &2) 0)
     solution)
