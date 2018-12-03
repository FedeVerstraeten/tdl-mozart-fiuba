local WaitSome Xs X1 X2 L in
   proc{WaitSome Xs}
      case Xs of H|T then
	 thread {Wait H}{Browse H} end
	 thread {Wait T}{Browse T} end
      end
   end

   L = [X1 X2]
   {WaitSome L}
   X2 = 1
   X1= 10
end
