(function() {
  'use strict';
  var CND, POWIERZA, alert, badge, debug, echo, help, info, rpr, urge, warn, whisper;

  //###########################################################################################################
  CND = require('cnd');

  rpr = CND.rpr;

  badge = 'POWIERZA';

  debug = CND.get_logger('debug', badge);

  alert = CND.get_logger('alert', badge);

  whisper = CND.get_logger('whisper', badge);

  warn = CND.get_logger('warn', badge);

  help = CND.get_logger('help', badge);

  urge = CND.get_logger('urge', badge);

  info = CND.get_logger('info', badge);

  echo = CND.echo.bind(CND);

  POWIERZA = require('../../pkg');

  //-----------------------------------------------------------------------------------------------------------
  this._set_globals = function() {
    var _alert;
    _alert = alert;
    globalThis.alert = (...P) => {
      alert(_border);
      alert();
      _alert(CND.reverse(...P));
      alert();
      return alert(_border);
    };
    globalThis.help = help;
    globalThis.urge = urge;
    globalThis.info = info;
    globalThis.debug = debug;
    return null;
  };

  //-----------------------------------------------------------------------------------------------------------
  this.demo = function() {
    debug('^1^', POWIERZA.powierza('abc', 'a_b-c'));
    return null;
  };

  //###########################################################################################################
  if (module === require.main) {
    (() => {
      this.demo();
      return null;
    })();
  }

}).call(this);

//# sourceMappingURL=main.js.map