; Solved 6.1

(defn member? (e lst)
  (cond
    ((nil? list) #f)
    ((= (fst lst) e) #t)
    (else (member? e (rst lst)))))

(defn pandigital? (a b c)
  (= (sort (append (append (digits a) (digits b)) (digits c)))
     (list 1 2 3 4 5 6 7 8 9)))

(def range1 (range 1 9))
(def range2 (range 12 98))
(def range3 (range 123 987))
(def range4 (range 1234 9876))

(defn find-pandigital (ra rb)
  (find-pandigital_ ra rb ra '()))

(defn find-pandigital_ (ra rb ra-backup acc)
  (cond
    ((nil? rb) acc)
    ((nil? ra)
     (find-pandigital_ ra-backup (rst rb) ra-backup acc))
    (else
      (let* ((a (fst ra)) (b (fst rb)) (c (* a b)))
        (if (pandigital? a b c)
          (find-pandigital_ (rst ra) rb ra-backup (cons c acc))
          (find-pandigital_ (rst ra) rb ra-backup acc))))))

(defn dedup (arr)
      (dedup_ arr -1 '()))

(defn dedup_ (arr last acc)
  (cond
    ((nil? arr) acc)
    ((= last (fst arr)) (dedup_ (rst arr) last acc))
    (else 
      (dedup_ (rst arr) (fst arr) (cons (fst arr) acc))
      )))

; The product of a n-digit number and a m-digit number
; is either n+m-1 and n+m digits long,
; so the only possible combinations (with n <= m) are
; 1.  1 x 4 = 4
; 2.  2 x 3 = 4

(~>
  (find-pandigital range1 range4)
  (append (find-pandigital range2 range3))
  sort
  dedup
  sum
  (println "Solution: "))
