(function() {
  'use strict';
  var CND, POWIERZA, alert, badge, debug, echo, help, info, rpr, sim, to_width, urge, warn, whisper;

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

  ({to_width} = require('to-width'));

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
  sim = function(a, b) {
    return Math.min(POWIERZA.powierza(a, b), POWIERZA.osa(a, b));
  };

  //-----------------------------------------------------------------------------------------------------------
  this.demo = function() {
    var a, b, i, len, pair, pairs, probes, show;
    this._set_globals();
    show = function(a, b) {
      var pc;
      pc = POWIERZA.powierza(a, b);
      // pc /= Math.max ( Array.from b ).length, 0
      // pc /= Math.max ( Array.from a ).length, ( Array.from b ).length
      echo(to_width(rpr(a), 20), to_width(rpr(b), 20), CND.yellow(to_width(rpr(pc), 10)), CND.yellow(to_width(rpr(POWIERZA.osa(a, b)), 10)), CND.yellow(to_width(rpr(sim(a, b)), 10)));
      return null;
    };
    probes = ['', '(⿱宀珎貝)', '12345', '123456789', 'a-b-c', 'a-b-c-d-e-f', 'a-bc', 'a..........b....', 'a..........b....c', 'ab-c', 'abc', 'abc', 'abcdef', '宀', '宀珎貝', '宀貝', '宀貝珎'];
    pairs = ((function() {
      var i, len, results;
      results = [];
      for (i = 0, len = probes.length; i < len; i++) {
        b = probes[i];
        results.push((function() {
          var j, len1, results1;
          results1 = [];
          for (j = 0, len1 = probes.length; j < len1; j++) {
            a = probes[j];
            if (a !== b) {
              results1.push([a, b]);
            }
          }
          return results1;
        })());
      }
      return results;
    })()).flat();
    pairs.sort(function(a, b) {
      // ( POWIERZA.powierza a... ) - ( POWIERZA.powierza b... )
      // ( POWIERZA.osa a... ) - ( POWIERZA.osa b... )
      return (sim(...a)) - (sim(...b));
    });
    for (i = 0, len = pairs.length; i < len; i++) {
      pair = pairs[i];
      show(...pair);
    }
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