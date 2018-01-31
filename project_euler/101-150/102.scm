; Solved: 26.1.18

(defn parse-triangle (points)
  (list
    (cons
      (cons (fst points) (frst points))
      (cons (frrst points) (frrrst points)))
    (cons
      (cons (fst (rrst points)) (frst (rrst points)))
      (cons (frrst (rrst points)) (frrrst (rrst points))))
    (cons
      (cons (fst (rrrrst points)) (frst (rrrrst points)))
      (cons (fst points) (frst points)))
     ))


; Check if a ray from (0, 0) with direction (0, 1)
; would hit the line
(defn hits-line? (line)
      (let ([x1 (ffst line)] [y1 (rfst line)]
            [x2 (frst line)] [y2 (rrst line)])
        (if (= x1 x2)
          #f
          (let ([alpha (/ x2 (- x2 x1))])
            (if (or (< alpha 0) (> alpha 1))
              #f
              (> (+ (* y1 alpha) (* y2 (- 1 alpha))) 0))))))

(defn origin-inside-triangle? (t)
      (= (count hits-line? t) 1))

(def triangles
     (~> "./project_euler/input-files/102.txt"
         file-read
         lines
         (reject empty?)
         (map &(~> &1 (string-split ",") (map string->number) parse-triangle))
         (count origin-inside-triangle?)
         (println "Solution: ")))
