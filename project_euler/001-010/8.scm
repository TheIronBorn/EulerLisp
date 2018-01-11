; Solved: 18.12.17
; Changes:
;  * implement string-bytes method
;  * implement filter
; 29.12.17, Refactor to use chunks & product methods
; 11.1.18, Refactor to use ~> 

(defn parse-byte (b) (- b 48))
(defn is-number (b) (and (>= b 48) (<= b 57)))

 (~> "project_euler/001-010/8.txt"
     file-read
     string-bytes
     (select is-number)
     (map parse-byte)
     (chunks 13)
     (map product)
     (apply max)
     (println "Solution: "))
