; Solved: 28.1.18
; Time: 5:59
; Solution: 2868868

(defn intersection (x1 y1 x2 y2 x3 y3 x4 y4)
  (let ([mx12 (- x1 x2)] [mx34 (- x3 x4)]
        [my12 (- y1 y2)] [my34 (- y3 y4)]
        [px12 (* x1 x2)] [px34 (* x3 x4)]
        [py12 (* y1 y2)] [py34 (* y3 y4)])
    (let ([den (- (* mx12 my34) (* my12 mx34))]
          [n1 (- (* x1 y2) (* y1 x2))]
          [n2 (- (* x3 y4) (* y3 x4))])
      (if (zero? den)
        '()
        (let ([px (/ (- (* n1 mx34) (* n2 mx12)) den)]
              [py (/ (- (* n1 my34) (* n2 my12)) den)])
              (let ([u
                      (if (zero? mx12)
                          (/ (- py y2) my12)
                          (/ (- px x2) mx12))]
                    [v
                      (if (zero? mx34)
                          (/ (- py y4) my34)
                          (/ (- px x4) mx34))])
                (if (and (> u 0) (< u 1) (> v 0) (< v 1))
                    (pair px py)
                    '())))))))

(defn make-blum ()
  (let ([s 290797])
      (fn ()
          (set! s (modexp s 2 50515093))
          (% s 500))))

(def blum (make-blum))
(def mylines
  (~> (range~ 1 5000)
      (map~ (fn (_) (list (blum) (blum) (blum) (blum))))
      collect
      list->vector))

(~>
  (range~ 0 4998)
  (map~
    (fn (ai)
        (let ([a (vector-ref mylines ai)])
          (~>
            (range~ (inc ai) 4999)
            (map~ (fn (bi)
              (let ([b (vector-ref mylines bi)])
                (intersection
                  (fst a) (frst a)
                  (frrst a) (frrrst a)
                  (fst b) (frst b)
                  (frrst b) (frrrst b)))))
            (select~ &(not (nil? &1)))
            collect))))
  collect
  flatten
  sort
  uniq
  length
  (println "Solution: "))
