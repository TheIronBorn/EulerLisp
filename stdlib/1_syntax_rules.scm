; (defsyntax and () (
;   ((and) #t)
;   ((and test) test)
;   ((and test1 test2 ...)
;    (if test1 (and test2 ...) #f))))
(defsyntax and () (
  ((and) #t)
  ((and test1) (if test1 #t #f))
  ((and test1 test2 ...)
   (if test1 (and test2 ...) #f))))

; TODO: The additional `let` makes this much slower
; (defsyntax or () (
;   ((or) #f)
;   ((or test) test)
;   ((or test1 test2 ...)
;    (let (x test1)
;    (if x x (or test2 ...))))))
(defsyntax or () (
  ((or) #f)
  ((or test1) (if test1 #t #f))
  ((or test1 test2 ...)
   (if test1 #t (or test2 ...)))))


; TODO: R5RS has a version that is more complex,
; does it have any features that would be nice to replicate?
(defsyntax cond () (
  ((cond else consq) consq)
  ((cond test consq) (if test consq))
  ((cond test consq rest ...)
     (if test
       consq
       (cond rest ...)))))

; TODO: R5RS has a version that is more complex,
; does it have any features that would be nice to replicate?
(defsyntax case () (
  ((case (key ...) clauses ...)
   (let (atom-key (key ...))
     (case atom-key clauses ...)))
  ((case key else consq) consq)
  ((case key atom consq) (if (= key atom) consq))
  ((case key atom consq rest ...)
   (if (= key atom)
       consq
       (case key rest ...)))))

(defsyntax defn () (
  ((defn name args body ...)
   (def name (fn args body ...)))))

(defsyntax when () (
  ((when test body ...)
   (if test (do body ...)))))


(defsyntax ~> () (
  ((~> first) first)
  ((~> first (second args ...))
   (second args ... first))
  ((~> first second)
   (second first))
  ((~> first (second args ...) rest ...)
   (~> (second args ... first) rest ...))
  ((~> first second rest ...)
   (~> (second first) rest ...))))
