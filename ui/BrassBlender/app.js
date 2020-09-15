(function () {

    if (typeof web3 !== 'undefined') {
        // Use Mist/MetaMask's provider
        web3js = new Web3(web3.currentProvider);
    } else {
        console.log('No web3? You should consider trying MetaMask!')
        // fallback - use your fallback strategy (local node / hosted node + in-dapp id mgmt / fail)
        web3js = new Web3(new Web3.providers.HttpProvider("http://localhost:8545"));
    }


    angular.module('gu')
        .run(function (pluginManager) {

            pluginManager.addActivator({
                id: 'c22b87d4-4c97-11e9-88ee-0f230ce0986e',
                name: 'Brass Worker - rendering',
                iconClass: 'glyphicon glyphicon-asterisk',
                sessionTag: ['gu:brass', 'gu:brass:taskType=Blender'],
                controller: controller
            });

            function controller(action, context, session) {
                switch (action) {
                    case 'new-session':
                        newSession(context);
                        break;
                    case 'browse':
                        context.setPage(session, '/plug/BrassBlender/selectprov.html');
                        break;
                }
            }

            function newSession(context) {
                context.setPage(undefined, '/plug/BrassBlender/base.html')
            }

            $(function () {
                $('<link href="https://gitcdn.github.io/bootstrap-toggle/2.2.2/css/bootstrap-toggle.min.css" rel="stylesheet">').appendTo($('head'));
            });
        })
        .controller('BrassBlenderBase', function ($scope, $log, $q, sessionMan) {

            $scope.dockerMode = true;
            $scope.config = {
                account: '',
                davUrl: 'http://127.0.0.1:55011',
                gwUrl: 'http://127.0.0.1:55001/',
            };

            $scope.gntInfo = {
                gnt: '-',
                gntb: '-'
            };

            $scope.accounts = function () {
                return gueth.accounts();
            };

            $scope.selectAccount = (account) => {
                //$scope.config.account = account.toString();
                $scope.fc.account.$setViewValue(account.toString());
                $scope.fc.account.$render();


                console.log('account', account, 'gnt', $scope.gntInfo);
            };

            $scope.$watch('config.account', (account, oldVal) => {
                $scope.gntInfo = {
                    gnt: '-',
                    gntb: '-'
                };

                if (gueth.checkAddress(account)) {
                    gueth.getBalance(gueth.gnt, account).then(b => {
                        $scope.$apply(function () {
                            $scope.gntInfo.gnt = b.toFixed(18);
                        })
                    });

                    gueth.getBalance(gueth.gntb, account).then(b => $scope.$apply(function () {
                        $scope.gntInfo.gntb = b.toFixed(18);
                    }));
                }

            });

            $scope.net = gueth.net;

            $scope.toggleMode = function () {
                $scope.dockerMode = !$scope.dockerMode;
            };


            $scope.goNext = function () {
                console.log('fc=', $scope.fc);
                if ($scope.fc.$invalid) {
                    return;
                }

                let context = $scope.$eval('sessionContext');

                sessionMan.create('Blendering for ' + $scope.config.account, ['gu:brass', 'gu:brass:taskType=Blender'])
                    .then(session => session.setConfig({
                        account: $scope.config.account,
                        davUrl: $scope.config.davUrl,
                        gwUrl: $scope.config.gwUrl,
                        docker: $scope.dockerMode
                    }).then(data => {
                        console.log('set-config', data);
                        return session;
                    }))
                    .then(session => {
                        context.setPage(session, '/plug/BrassBlender/selectprov.html');
                    })


            };

            $scope.isConnected = () => web3js.isConnected();

        })
        .controller('BrassBlenderSelectProv', function ($scope, $http) {
            let session = $scope.$eval('currentSession');
            let context = $scope.$eval('sessionContext');

            session.getConfig().then(config => {
                if (config.status === "working") {
                    context.setPage(session, '/plug/BrassBlender/work.html');
                }
            });

            session.peers().then(peers => {
                $scope.sessionPeers = peers;
            });

            $scope.goNext = function () {
                session.addPeers($scope.sessionPeers)
                    .then(function() {
                        $http.post('/service/local/BrassBlender/gu-blender-mediator/gw', session.id).then(function() {
                            context.setPage(session, '/plug/BrassBlender/work.html');
                        })
                    });
            };

        })
        .controller('BrassBlenderWork', function ($scope, $http, $interval) {
            let session = $scope.$eval('currentSession');
            let context = $scope.$eval('sessionContext');

            $scope.working = true;
            $scope.sessionConfig = null;
            $scope.sessionPeers = [];

            $scope.gntInfo = {
                gnt: '-',
                gntb: '-'
            };

            function refreshAccount(account) {
                if (gueth.checkAddress(account)) {
                    gueth.getBalance(gueth.gnt, account).then(b => {
                        $scope.$apply(function () {
                            $scope.gntInfo.gnt = b.toFixed(18);
                        })
                    });

                    gueth.getBalance(gueth.gntb, account).then(b => $scope.$apply(function () {
                        $scope.gntInfo.gntb = b.toFixed(18);
                    }));
                }

            }

            $scope.restartSession = function () {
                $http.post('/service/local/BrassBlender/gu-blender-mediator/gw', session.id).then(function() {
                    $scope.working = true;
                    refresh();

                });
            };


            function refresh() {
                session.getConfig().then(c => {
                    $scope.sessionConfig = c;
                    refreshAccount(c.account);
                });
                $http.get(`/service/local/BrassBlender/gu-blender-mediator/gw/${session.id}`).then(c => {
                    $scope.sessionStats = c.data;
                    $scope.working = true;
                }, e => {
                    $scope.working = false;
                });

            }

            session.peers().then(peers => {
                $scope.sessionPeers = peers;
            });

            let stop = $interval(function() {
                refresh()
            }, 5000);

            $scope.$on('$destroy', function() {
                $interval.cancel(stop);
                stop = undefined;
            });

            refresh();

        })

        .directive('ethAddress', function () {
            return {
                require: 'ngModel',
                link: function (scope, element, attr, mCtrl) {
                    function myValidation(value) {
                        if (gueth.checkAddress(value)) {
                            mCtrl.$setValidity('charE', true);
                        } else {
                            mCtrl.$setValidity('charE', false);
                        }
                        return value;
                    }

                    mCtrl.$parsers.push(myValidation);
                }
            };
        });

})();
