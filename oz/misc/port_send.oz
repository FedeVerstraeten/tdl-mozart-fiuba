local Puerto S Enviar Recibir in
  proc {Enviar N Puerto Limit}
  if N < Limit then {Send Puerto N}{Enviar N+1 Puerto Limit}
  end
  end
  proc {Recibir S}
    case S of H|T then {Browse H} {Recibir T} end
  end
  {NewPort S Puerto}
  thread {Recibir S} end
  thread {Enviar 1 Puerto 10} end
end
