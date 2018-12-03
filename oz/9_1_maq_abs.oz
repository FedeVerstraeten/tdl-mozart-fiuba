local MyMap L Lout A B C in
   fun {MyMap Xs F}
      case Xs
      of nil then nil
      [] X|Xr then {F X}|{MyMap Xr F}
      end
   end
   L=[1 2 3 4]
   Lout = {MyMap L fun lazy {$ X} X*X end }
   A = Lout.1
   B = Lout.2.1
   C = A + B
   {Browse C}
end